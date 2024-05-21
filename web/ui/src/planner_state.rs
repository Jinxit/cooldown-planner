use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use itertools::Itertools;

use auto_battle_net::LocalizedString;
use fight_domain::{Attack, AttackUuid, CharacterUuid, Lookup, SpellUuid};
use optimizer::{Assignment, AssignmentState, AssignmentUuid};

use crate::{PlannerCharacter, PlannerCharacterTemplate, PlannerFight};

#[derive(Clone)]
pub struct PlannerState {
    fights: Vec<Arc<dyn PlannerFight>>,
    selected_fight_index: usize,
    characters: Lookup<PlannerCharacter>,
}

impl PlannerState {
    pub fn new(fights: Vec<Arc<dyn PlannerFight>>) -> Self {
        Self {
            fights,
            selected_fight_index: 0,
            characters: [PlannerCharacter::new(
                CharacterUuid::general(),
                PlannerCharacterTemplate::General,
            )]
            .into_iter()
            .collect(),
        }
    }

    pub fn add_ui_character(&mut self, new_character: PlannerCharacterTemplate) {
        self.characters
            .put(PlannerCharacter::new(CharacterUuid::new(), new_character));
    }

    pub fn replace_ui_character(
        &mut self,
        prev_uuid: CharacterUuid,
        new_character: PlannerCharacterTemplate,
    ) {
        let character = PlannerCharacter::new(CharacterUuid::new(), new_character);
        self.characters.replace(&prev_uuid, character);
    }

    pub fn remove_ui_character(&mut self, uuid: CharacterUuid) {
        self.characters.take(&uuid);
    }

    pub fn update_assignment_suggestions(&mut self, suggestions: Lookup<Assignment>) {
        for (character_uuid, character_suggestions) in
            &suggestions.into_iter().into_group_map_by(|a| a.character)
        {
            let mut character = self.characters.get_mut(*character_uuid).unwrap();
            character.suggested_assignments = character_suggestions.into_iter().collect();
        }
    }

    pub fn toggle_assignment(&mut self, assignment: &Assignment) {
        let is_spell_assignable = self.is_spell_assignable(assignment.clone());
        let mut ui_character = self.characters.get_mut(assignment.character).unwrap();
        match (assignment.state, is_spell_assignable) {
            (AssignmentState::Forced, _) => {
                // remove currently forced assignment
                ui_character.assignments.take(&assignment.uuid);
            }
            (_, true) => {
                // create new forced assignment
                ui_character.assignments.put(assignment.clone());
            }
            (_, false) => {
                // do nothing
            }
        };
    }

    pub fn lock_suggestions(&mut self) {
        for character in self.characters.iter_mut() {
            for assignment in character.assignments.iter_mut() {
                if assignment.state == AssignmentState::Suggested {
                    assignment.state = AssignmentState::Forced;
                }
            }
        }
    }

    pub fn set_character_class(&mut self, uuid: CharacterUuid, class: Option<LocalizedString>) {
        self.characters.get_mut(uuid).unwrap().class = class;
    }

    pub fn set_character_spec(&mut self, uuid: CharacterUuid, spec: Option<LocalizedString>) {
        self.characters.get_mut(uuid).unwrap().spec = spec;
    }

    pub fn toggle_spell_enabled(&mut self, character: CharacterUuid, spell: SpellUuid) {
        let mut character = self.characters.get_mut(character).unwrap();
        let mut spell = character.spells.get_mut(spell).unwrap();
        spell.enabled = !spell.enabled;
    }

    pub fn set_selected_fight_index(&mut self, index: usize) {
        self.selected_fight_index = index;
    }

    pub fn is_spell_assignable(&self, assignment: Assignment) -> bool {
        let character_uuid = assignment.character;
        let attack_uuid = assignment.attack;
        let spell_uuid = assignment.spell;
        let attacks = self.attacks();

        let spells = &self.characters.get(&character_uuid).unwrap().spells;
        let spell = spells.get(&spell_uuid).unwrap();
        let attack = &attacks.get(&attack_uuid).unwrap();

        let forced_assignments_for_character = self
            .locked_assignments()
            .into_iter()
            .filter(|a| {
                a.character != character_uuid && a.attack != attack_uuid && a.spell != spell_uuid
            })
            .cloned()
            .collect::<Lookup<_>>();

        let exclusive_with_spell_uuids = spells
            .iter()
            .filter(|s| spell.exclusive_with.contains(&s.identifier))
            .map(|spell| spell.uuid)
            .collect::<HashSet<_>>();

        let has_assigned_exclusives = forced_assignments_for_character
            .iter()
            .any(|assignment| exclusive_with_spell_uuids.contains(&assignment.spell));

        if has_assigned_exclusives {
            return false;
        }

        let other_usages = forced_assignments_for_character
            .iter()
            //.filter(|other| other.uuid != assignment_uuid)
            .filter(|assignment| {
                spells
                    .get(&assignment.spell)
                    .map(|assigned_spell| assigned_spell.identifier == spell.identifier)
                    .unwrap_or(false)
            })
            .cloned()
            .collect::<Vec<_>>();

        if other_usages
            .iter()
            .any(|other_usage| other_usage.spell != spell.uuid)
        {
            return false;
        }

        let attack_timer = attack.timer.static_timer();
        let other_usages_within_cooldown = other_usages
            .iter()
            .filter(|other_usage| {
                let other_timer = attacks.get(&other_usage.attack).unwrap().timer.clone();
                other_timer.static_timer().abs_diff(attack_timer) <= spell.cooldown
            })
            .count();

        if other_usages_within_cooldown >= spell.charges {
            return false;
        }

        true
    }

    pub fn all_assignments(&self) -> &HashMap<(CharacterUuid, AttackUuid), Lookup<Assignment>> {
        let base = self
            .characters
            .iter()
            .filter(|c| c.name.is_some())
            .flat_map(|c| {
                c.spells.iter().flat_map(|s| {
                    self.attacks().iter().map(|a| Assignment {
                        uuid: AssignmentUuid::new(),
                        character: c.uuid,
                        spell: s.uuid,
                        attack: a.uuid,
                        state: AssignmentState::Unassigned,
                    })
                })
            });
        let suggested = self.suggested_assignments().into_iter();
        let locked = self
            .characters
            .iter()
            .filter(|c| c.name.is_some())
            .flat_map(|c| c.assignments.iter())
            .cloned();

        base.chain(suggested)
            .chain(locked)
            .map(|a| ((a.character, a.attack), a))
            .into_grouping_map()
            .collect()
    }

    pub fn attacks(&self) -> &Lookup<Attack> {
        &self
            .selected_fight()
            .map(|f| f.attacks())
            .unwrap_or_default()
    }

    pub fn selected_fight(&self) -> Option<Arc<dyn PlannerFight>> {
        self.fights.get(self.selected_fight_index).cloned()
    }

    pub fn fights(&self) -> &Vec<Arc<dyn PlannerFight>> {
        &self.fights
    }

    pub fn locked_assignments(&self) -> &Lookup<Assignment> {
        self.characters
            .iter()
            .filter(|c| c.name.is_some())
            .flat_map(|c| c.assignments.iter())
            .filter(|a| a.state == AssignmentState::Forced)
            .map(|a| Assignment {
                uuid: a.uuid,
                character: a.character,
                spell: a.spell,
                attack: a.attack,
                state: a.state,
            })
            .collect()
    }

    pub fn characters(&self) -> &Lookup<PlannerCharacter> {
        &self.characters
    }
}
