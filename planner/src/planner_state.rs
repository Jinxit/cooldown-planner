use std::collections::HashSet;
use std::sync::Arc;

use itertools::Itertools;

use fight_domain::{Attack, AttackUuid, CharacterUuid, Lookup, SpellUuid, TimeStep};
use i18n::{Locale, LocalizedString};
use optimizer::{Assignment, AssignmentState};

use crate::{AsInGameNote, PlannerCharacter, PlannerCharacterTemplate, PlannerFight};

#[derive(Clone)]
pub struct PlannerState {
    fights: Vec<Arc<dyn PlannerFight>>,
    selected_fight_index: usize,
    characters: Lookup<PlannerCharacter>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Assignability {
    Assignable,
    HasAssignedExclusives,
    HasOtherUsageOfSpellVariant,
    HasOtherUsageWithinCooldown,
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

    pub fn add_character(&mut self, new_character: PlannerCharacterTemplate) -> CharacterUuid {
        if matches!(new_character, PlannerCharacterTemplate::General) {
            panic!("Cannot add general character");
        }
        let uuid = CharacterUuid::new();
        // keep general character last
        self.characters.insert(
            self.characters.len() - 1,
            PlannerCharacter::new(uuid, new_character),
        );
        uuid
    }

    pub fn replace_character(
        &mut self,
        prev_uuid: CharacterUuid,
        new_character: PlannerCharacterTemplate,
    ) -> CharacterUuid {
        if prev_uuid == CharacterUuid::general() {
            panic!("Cannot replace general character");
        } else if matches!(new_character, PlannerCharacterTemplate::General) {
            panic!("Cannot replace with new general character");
        }
        let uuid = CharacterUuid::new();
        let character = PlannerCharacter::new(uuid, new_character);
        self.characters.replace(&prev_uuid, character);
        uuid
    }

    pub fn remove_character(&mut self, uuid: CharacterUuid) {
        if uuid == CharacterUuid::general() {
            panic!("Cannot remove general character");
        }
        self.characters.take(&uuid);
    }

    pub fn replace_assignment_suggestions(&mut self, assignments: Lookup<Assignment>) {
        for character in self.characters.iter_mut() {
            character.assignments.replace_suggestions(
                assignments
                    .iter()
                    .filter(|a| {
                        a.character == character.uuid && a.state == AssignmentState::Suggested
                    })
                    .map(|a| (a.spell, a.attack)),
            );
        }
    }

    pub fn toggle_assignment(
        &mut self,
        character: CharacterUuid,
        spell: SpellUuid,
        attack: AttackUuid,
    ) {
        let state = self
            .characters
            .get(&character)
            .unwrap()
            .assignment_state(spell, attack);

        if state == AssignmentState::Locked {
            // remove currently locked assignment
            let character = self.characters.get_mut(&character).unwrap();
            character.assignments.unlock(spell, attack);
        } else {
            // need to check if the spell is actually assignable
            let is_spell_assignable = self.is_spell_assignable(character, spell, attack);
            if matches!(is_spell_assignable, Assignability::Assignable) {
                // create new locked assignment
                let character = self.characters.get_mut(&character).unwrap();
                character.assignments.assign_locked(spell, attack);
            }
        }
    }

    pub fn lock_suggestions(&mut self) {
        for character in self.characters.iter_mut() {
            character.assignments.lock_suggestions();
        }
    }

    pub fn change_character_class(&mut self, uuid: CharacterUuid, class: LocalizedString) {
        if uuid == CharacterUuid::general() {
            panic!("Cannot change the class of the general character");
        }
        let character = self.characters.get_mut(&uuid).unwrap();
        if Some(&class) != character.class.as_ref() {
            character.change_class(class);
        }
    }

    pub fn change_character_spec(&mut self, uuid: CharacterUuid, spec: LocalizedString) {
        if uuid == CharacterUuid::general() {
            panic!("Cannot change the spec of the general character");
        }
        let character = self.characters.get_mut(&uuid).unwrap();
        if Some(&spec) != character.spec.as_ref() {
            character.change_spec(spec);
        }
    }

    pub fn toggle_spell_enabled(&mut self, character: CharacterUuid, spell: SpellUuid) {
        let character = self.characters.get_mut(&character).unwrap();
        character.toggle_spell_enabled(spell);
    }

    pub fn set_selected_fight_index(&mut self, index: usize) {
        if index >= self.fights.len() {
            panic!("selected fight index out of bounds");
        }
        self.selected_fight_index = index;
    }

    pub fn is_spell_assignable(
        &self,
        character_uuid: CharacterUuid,
        spell_uuid: SpellUuid,
        attack_uuid: AttackUuid,
    ) -> Assignability {
        let attacks = self.attacks();

        let character = self.characters.get(&character_uuid).unwrap();
        let spells = &character.spells;
        let spell = spells.get(&spell_uuid).unwrap();
        let attack = &attacks.get(&attack_uuid).unwrap();

        let exclusive_with_spell_uuids = spells
            .iter()
            .filter(|s| spell.exclusive_with.contains(&s.identifier))
            .map(|spell| spell.uuid)
            .collect::<HashSet<_>>();

        let has_assigned_exclusives = character
            .assignments
            .locked()
            .any(|assignment| exclusive_with_spell_uuids.contains(&assignment.0));

        if has_assigned_exclusives {
            return Assignability::HasAssignedExclusives;
        }

        let other_usages = character
            .assignments
            .locked()
            .filter(|(other_spell, other_attack)| *other_spell != spell_uuid || *other_attack != attack_uuid)
            .filter(|assignment| {
                spells
                    .get(&assignment.0)
                    .map(|assigned_spell| assigned_spell.identifier == spell.identifier)
                    .unwrap_or(false)
            })
            .cloned()
            .collect::<Vec<_>>();

        if other_usages
            .iter()
            .any(|other_usage| other_usage.0 != spell.uuid)
        {
            return Assignability::HasOtherUsageOfSpellVariant;
        }

        let attack_timer = attack.timer.static_timer();
        let other_usages_within_cooldown = other_usages
            .iter()
            .filter(|other_usage| {
                let other_timer = attacks.get(&other_usage.1).unwrap().timer.clone();
                other_timer.static_timer().abs_diff(attack_timer) <= spell.cooldown
            })
            .count();

        if other_usages_within_cooldown >= spell.charges {
            return Assignability::HasOtherUsageWithinCooldown;
        }

        Assignability::Assignable
    }

    pub fn attacks(&self) -> Lookup<Attack> {
        self.fights
            .get(self.selected_fight_index)
            .map(|f| f.attacks())
            .unwrap_or_default()
    }

    pub fn selected_fight(&self) -> Option<Arc<dyn PlannerFight>> {
        self.fights.get(self.selected_fight_index).cloned()
    }

    pub fn fights(&self) -> &Vec<Arc<dyn PlannerFight>> {
        &self.fights
    }

    pub fn characters(&self) -> &Lookup<PlannerCharacter> {
        &self.characters
    }

    pub fn locked_assignments(&self) -> Lookup<Assignment> {
        self.characters
            .iter()
            .flat_map(|character| character.assignments.locked().map(move |(spell, attack)| {
                Assignment::new(character.uuid, *spell, *attack, AssignmentState::Locked)
            }))
            .collect::<Vec<_>>()
            .into_iter()
            .collect()
    }

    pub fn export(&self) -> Option<String> {
        let selected_fight = self.selected_fight()?;
        let attacks = selected_fight.attacks();
        Some(self.characters
            .iter()
            .filter(|character| character.name.is_some())
            .flat_map(|character| character.assignments.all().map(move |(spell, attack)| (character, spell, attack)))
            .sorted_by_key(|(_, _, attack)| attacks.get(attack).unwrap().timer.static_timer())
            .chunk_by(|(_, _, attack)| **attack)
            .into_iter()
            .map(|(attack_uuid, assignments)| {
                let characters = assignments
                    .sorted_by_key(|(character, _, _)| character.name.clone())
                    .chunk_by(|(character, _, _)| (character.uuid, character.name.clone().unwrap()))
                    .into_iter()
                    .map(|((_, character_name), assignments)| {
                        // TODO: How to do general properly?
                        [character_name.get(Locale::EnglishUnitedStates).to_owned()].into_iter().chain(assignments.into_iter().map(|(character, spell, _)| {
                            let identifier = character.spells.get(spell).unwrap().identifier.clone();
                            identifier.in_game_note().to_string()
                        })).join(" ")
                    })
                    .join("  ");
                let attack = attacks.get(&attack_uuid).unwrap();
                let dynamic_timer = attack.timer.dynamic_timer.unwrap_or(TimeStep::zero());
                let spell_trigger = match &attack.timer.dynamic_trigger_cleu_event {
                    Some(cleu_event) => {
                        let event_type = &cleu_event.r#type;
                        let event_id = cleu_event.event;
                        let counter = cleu_event.counter;
                        format!(",{event_type}:{event_id}:{counter}")
                    },
                    None => "".to_string(),
                };
                let static_timer = attack.timer.static_timer();
                let attack_name = &attack.name;
                format!("{{time:{dynamic_timer}{spell_trigger}}}{static_timer} - {attack_name} - {characters}")
            }).join("\r\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use uuid::uuid;

    use auto_battle_net::game_data::journal::{journal_encounter, journal_instance};
    use auto_battle_net::game_data::journal::journal_encounter::JournalEncounterResponse;
    use auto_battle_net::game_data::journal::journal_instance::JournalInstanceResponse;
    use auto_battle_net::Link;
    use dragonflight::aberrus::kazzara::Kazzara;
    use fight_domain::AttackUuid;
    use i18n::LocalizedString;

    use crate::fights::dragonflight;
    use crate::PlannerRealm;

    use super::*;

    #[test]
    fn default_state() {
        let state = PlannerState::new(vec![]);
        assert_eq!(state.fights().len(), 0);
        assert_eq!(state.selected_fight_index, 0);
        assert_eq!(state.characters().len(), 1);
    }

    #[test]
    fn add_character() {
        let mut state = PlannerState::new(vec![]);
        let uuid_added = state.add_character(PlannerCharacterTemplate::Unknown);
        assert_eq!(state.characters().len(), 2);

        let characters = state.characters().into_iter().collect::<Vec<_>>();
        assert_eq!(characters[0].uuid, uuid_added);
        assert_eq!(characters[1].uuid, CharacterUuid::general());
    }

    #[test]
    #[should_panic]
    fn add_general_character_panics() {
        let mut state = PlannerState::new(vec![]);
        state.add_character(PlannerCharacterTemplate::General);
    }

    #[test]
    fn replace_character() {
        let mut state = PlannerState::new(vec![]);
        let uuid_added = state.add_character(PlannerCharacterTemplate::Unknown);
        let uuid_replaced = state.replace_character(
            uuid_added,
            PlannerCharacterTemplate::Custom {
                name: "Test".to_string(),
            },
        );
        assert_eq!(state.characters().len(), 2);

        let characters = state.characters().into_iter().collect::<Vec<_>>();
        assert_eq!(characters[0].uuid, uuid_replaced);
        assert_eq!(characters[1].uuid, CharacterUuid::general());
    }

    #[test]
    #[should_panic]
    fn replace_general_character_panics() {
        let mut state = PlannerState::new(vec![]);
        state.replace_character(
            CharacterUuid::general(),
            PlannerCharacterTemplate::Custom {
                name: "Test".to_string(),
            },
        );
    }

    #[test]
    #[should_panic]
    fn replace_with_new_general_character_panics() {
        let mut state = PlannerState::new(vec![]);
        let uuid_added = state.add_character(PlannerCharacterTemplate::Unknown);
        state.replace_character(uuid_added, PlannerCharacterTemplate::General);
    }

    #[test]
    #[should_panic]
    fn change_class_of_general_character_panics() {
        let mut state = PlannerState::new(vec![]);
        state.change_character_class(CharacterUuid::general(), LocalizedString::constant("Druid"));
    }

    #[test]
    #[should_panic]
    fn change_spec_of_general_character_panics() {
        let mut state = PlannerState::new(vec![]);
        state.change_character_spec(
            CharacterUuid::general(),
            LocalizedString::constant("Restoration"),
        );
    }

    #[test]
    fn remove_existing_character() {
        let mut state = PlannerState::new(vec![]);
        let uuid_added = state.add_character(PlannerCharacterTemplate::Unknown);
        state.remove_character(uuid_added);
        assert_eq!(state.characters().len(), 1);

        let characters = state.characters().into_iter().collect::<Vec<_>>();
        assert_eq!(characters[0].uuid, CharacterUuid::general());
    }

    #[test]
    fn remove_nonexisting_character() {
        let mut state = PlannerState::new(vec![]);
        let uuid = CharacterUuid::new();
        state.remove_character(uuid);
        assert_eq!(state.characters().len(), 1);

        let characters = state.characters().into_iter().collect::<Vec<_>>();
        assert_eq!(characters[0].uuid, CharacterUuid::general());
    }

    #[test]
    #[should_panic]
    fn remove_general_character_panics() {
        let mut state = PlannerState::new(vec![]);
        state.remove_character(CharacterUuid::general());
    }

    struct StateWithSuggestions {
        state: PlannerState,
        alice: CharacterUuid,
        alice_suggested_assignment_0: (SpellUuid, AttackUuid),
        alice_suggested_assignment_1: (SpellUuid, AttackUuid),
        bob: CharacterUuid,
        bob_suggested_assignment_0: (SpellUuid, AttackUuid),
        bob_suggested_assignment_1: (SpellUuid, AttackUuid),
    }

    const TRANQ_3M: SpellUuid = SpellUuid::new(uuid!("43d4698a-6d7a-4afe-bc21-5b175988e5e0"));
    const TRANQ_2M: SpellUuid = SpellUuid::new(uuid!("c3f8a190-2701-4bf7-87af-91ffb5ca969b"));
    const CONVOKE: SpellUuid = SpellUuid::new(uuid!("b8603366-ab57-413e-b6a9-a3c37af87a1c"));
    const TREE_OF_LIFE: SpellUuid = SpellUuid::new(uuid!("e561b6ff-077b-4ae5-97c7-395eab1062ef"));

    const ON_PULL: AttackUuid = AttackUuid::new(uuid!("755af363-d688-4147-9e30-7bf0f9bf00f9"));
    const AOE_80_PERCENT: AttackUuid =
        AttackUuid::new(uuid!("53d6e795-e1d1-4f79-a250-e2bfe07abbbd"));
    const KNOCK_AOE: AttackUuid = AttackUuid::new(uuid!("ec9bdd6b-ec2f-499a-9415-0ae043f20465"));

    fn create_state_with_suggestions() -> StateWithSuggestions {
        let mut state = PlannerState::new(vec![]);
        let alice = state.add_character(PlannerCharacterTemplate::Custom {
            name: "Alice".to_string(),
        });
        let bob = state.add_character(PlannerCharacterTemplate::Custom {
            name: "Bob".to_string(),
        });

        let alice_suggested_assignment_0 = (TRANQ_3M, ON_PULL);
        let alice_suggested_assignment_1 = (TRANQ_3M, AOE_80_PERCENT);
        let bob_suggested_assignment_0 = (TRANQ_2M, ON_PULL);
        let bob_suggested_assignment_1 = (TRANQ_2M, AOE_80_PERCENT);

        let suggested_assignments = [
            (alice, alice_suggested_assignment_0),
            (alice, alice_suggested_assignment_1),
            (bob, bob_suggested_assignment_0),
            (bob, bob_suggested_assignment_1),
        ]
        .into_iter()
        .map(|(character, (spell, attack))| Assignment {
            character,
            spell,
            attack,
            state: AssignmentState::Suggested,
        })
        .collect::<Lookup<_>>();

        state.replace_assignment_suggestions(suggested_assignments);

        StateWithSuggestions {
            state,
            alice,
            alice_suggested_assignment_0,
            alice_suggested_assignment_1,
            bob,
            bob_suggested_assignment_0,
            bob_suggested_assignment_1,
        }
    }

    #[test]
    fn replace_assignment_suggestions() {
        // given, when
        let s = create_state_with_suggestions();

        // then
        let alice_assignments = s
            .state
            .characters()
            .get(&s.alice)
            .unwrap()
            .assignments
            .suggested()
            .collect::<Vec<_>>();
        assert_eq!(alice_assignments.len(), 2);
        assert!(alice_assignments.contains(&&s.alice_suggested_assignment_0));
        assert!(alice_assignments.contains(&&s.alice_suggested_assignment_1));

        let bob_assignments = s
            .state
            .characters()
            .get(&s.bob)
            .unwrap()
            .assignments
            .suggested()
            .collect::<Vec<_>>();
        assert_eq!(bob_assignments.len(), 2);
        assert!(bob_assignments.contains(&&s.bob_suggested_assignment_0));
        assert!(bob_assignments.contains(&&s.bob_suggested_assignment_1));
    }

    #[test]
    fn clear_suggestions() {
        // given
        let mut s = create_state_with_suggestions();

        // when
        // giving new empty suggestions should clear previous suggestions
        s.state.replace_assignment_suggestions(Lookup::default());

        // then
        let alice_assignments = s
            .state
            .characters()
            .get(&s.alice)
            .unwrap()
            .assignments
            .suggested()
            .collect::<Vec<_>>();
        assert_eq!(alice_assignments.len(), 0);

        let bob_assignments = s
            .state
            .characters()
            .get(&s.bob)
            .unwrap()
            .assignments
            .suggested()
            .collect::<Vec<_>>();
        assert_eq!(bob_assignments.len(), 0);
    }

    fn mock_journal_instance() -> JournalInstanceResponse {
        let empty = LocalizedString::constant("");
        let link = Link {
            href: "".to_string(),
        };
        JournalInstanceResponse {
            links: Default::default(),
            area: None,
            category: journal_instance::Category {
                r#type: "".to_string(),
            },
            description: empty.clone(),
            encounters: vec![],
            expansion: journal_instance::Encounters {
                id: 0,
                key: link.clone(),
                name: empty.clone(),
            },
            id: 0,
            location: journal_instance::Area {
                id: 0,
                name: empty.clone(),
            },
            map: journal_instance::Area {
                id: 0,
                name: empty.clone(),
            },
            media: journal_instance::Media { id: 0, key: link },
            minimum_level: 0,
            modes: vec![],
            name: empty,
            order_index: 0,
        }
    }

    fn mock_journal_encounter() -> JournalEncounterResponse {
        let empty = LocalizedString::constant("");
        let link = Link {
            href: "".to_string(),
        };
        JournalEncounterResponse {
            links: Default::default(),
            category: journal_encounter::Category {
                r#type: "".to_string(),
            },
            creatures: vec![],
            description: empty.clone(),
            id: 0,
            instance: journal_encounter::Instance {
                id: 0,
                key: link.clone(),
                name: empty.clone(),
            },
            items: vec![],
            modes: vec![],
            name: empty.clone(),
            sections: vec![],
        }
    }

    #[test]
    fn toggle_assignment_from_unassigned_to_locked_and_back() {
        let mut state = PlannerState::new(vec![Arc::new(Kazzara::mythic(
            Some(&mock_journal_instance()),
            Some(&mock_journal_encounter()),
        ))]);
        let realm = PlannerRealm {
            name: LocalizedString::constant("Test"),
            slug: "test".to_string(),
        };
        let character = state.add_character(PlannerCharacterTemplate::Known {
            name: "Test".to_string(),
            realm,
            class: LocalizedString::constant("Druid"),
            spec: LocalizedString::constant("Restoration"),
        });

        assert_eq!(
            state
                .characters()
                .get(&character)
                .unwrap()
                .assignments
                .locked()
                .count(),
            0
        );

        state.toggle_assignment(character, TRANQ_3M, ON_PULL);

        assert_eq!(
            state
                .characters()
                .get(&character)
                .unwrap()
                .assignments
                .locked()
                .count(),
            1
        );
        let locked_assignment = state
            .characters()
            .get(&character)
            .unwrap()
            .assignments
            .locked()
            .next()
            .unwrap();
        assert_eq!(locked_assignment, &(TRANQ_3M, ON_PULL));

        state.toggle_assignment(character, TRANQ_3M, ON_PULL);

        assert_eq!(
            state
                .characters()
                .get(&character)
                .unwrap()
                .assignments
                .locked()
                .count(),
            0
        );
    }

    #[test]
    fn toggle_assignment_from_suggested_to_locked_and_back() {
        let mut state = PlannerState::new(vec![Arc::new(Kazzara::mythic(
            Some(&mock_journal_instance()),
            Some(&mock_journal_encounter()),
        ))]);
        let realm = PlannerRealm {
            name: LocalizedString::constant("Test"),
            slug: "test".to_string(),
        };
        let character_uuid = state.add_character(PlannerCharacterTemplate::Known {
            name: "Test".to_string(),
            realm,
            class: LocalizedString::constant("Druid"),
            spec: LocalizedString::constant("Restoration"),
        });

        {
            let character = state.characters().get(&character_uuid).unwrap();
            assert_eq!(character.assignments.suggested().count(), 0);
            assert_eq!(character.assignments.locked().count(), 0);
        }

        state.replace_assignment_suggestions(
            [Assignment {
                character: character_uuid,
                spell: TRANQ_3M,
                attack: ON_PULL,
                state: AssignmentState::Suggested,
            }]
            .into_iter()
            .collect(),
        );
        {
            let character = state.characters().get(&character_uuid).unwrap();
            assert_eq!(character.assignments.suggested().count(), 1);
            let suggested_assignment = character.assignments.suggested().next().unwrap();
            assert_eq!(suggested_assignment, &(TRANQ_3M, ON_PULL));
        }

        state.toggle_assignment(character_uuid, TRANQ_3M, ON_PULL);
        {
            let character = state.characters().get(&character_uuid).unwrap();
            // suggestion should still remain
            assert_eq!(character.assignments.suggested().count(), 1);
            let suggested_assignment = character.assignments.suggested().next().unwrap();
            assert_eq!(suggested_assignment, &(TRANQ_3M, ON_PULL));

            // but now there should also be a locked assignment
            assert_eq!(character.assignments.locked().count(), 1);
            let locked_assignment = character.assignments.locked().next().unwrap();
            assert_eq!(locked_assignment, &(TRANQ_3M, ON_PULL));
        }

        // toggling off again should remove the locked assignment but keep the suggestion
        state.toggle_assignment(character_uuid, TRANQ_3M, ON_PULL);
        {
            let character = state.characters().get(&character_uuid).unwrap();
            assert_eq!(character.assignments.locked().count(), 0);
            assert_eq!(character.assignments.suggested().count(), 1);
            let suggested_assignment = character.assignments.suggested().next().unwrap();
            assert_eq!(suggested_assignment, &(TRANQ_3M, ON_PULL));
        }
    }

    fn base_assignability_setup() -> (PlannerState, CharacterUuid) {
        let mut state = PlannerState::new(vec![Arc::new(Kazzara::mythic(
            Some(&mock_journal_instance()),
            Some(&mock_journal_encounter()),
        ))]);
        let realm = PlannerRealm {
            name: LocalizedString::constant("Test"),
            slug: "test".to_string(),
        };
        let character = state.add_character(PlannerCharacterTemplate::Known {
            name: "Test".to_string(),
            realm,
            class: LocalizedString::constant("Druid"),
            spec: LocalizedString::constant("Restoration"),
        });

        (state, character)
    }

    #[test]
    fn spell_assignability_with_no_assignments() {
        let (state, character) = base_assignability_setup();

        assert_eq!(
            state.is_spell_assignable(character, TRANQ_3M, ON_PULL),
            Assignability::Assignable
        );
    }

    #[test]
    fn spell_assignability_with_different_spell_locked() {
        let (mut state, character) = base_assignability_setup();

        state
            .characters
            .get_mut(&character)
            .unwrap()
            .assignments
            .assign_locked(TRANQ_2M, ON_PULL);

        assert_eq!(
            state.is_spell_assignable(character, CONVOKE, AOE_80_PERCENT),
            Assignability::Assignable
        );
    }

    #[test]
    fn spell_assignability_with_spell_variant_locked() {
        let (mut state, character) = base_assignability_setup();

        state
            .characters
            .get_mut(&character)
            .unwrap()
            .assignments
            .assign_locked(TRANQ_2M, ON_PULL);

        assert_eq!(
            state.is_spell_assignable(character, TRANQ_3M, AOE_80_PERCENT),
            Assignability::HasOtherUsageOfSpellVariant
        );
    }

    #[test]
    fn spell_assignability_with_same_spell_locked_within_cooldown() {
        let (mut state, character) = base_assignability_setup();

        state
            .characters
            .get_mut(&character)
            .unwrap()
            .assignments
            .assign_locked(TRANQ_2M, ON_PULL);

        assert_eq!(
            state.is_spell_assignable(character, TRANQ_2M, KNOCK_AOE),
            Assignability::HasOtherUsageWithinCooldown
        );
    }

    #[test]
    fn spell_assignability_with_exclusive_spell_locked() {
        let (mut state, character) = base_assignability_setup();

        state
            .characters
            .get_mut(&character)
            .unwrap()
            .assignments
            .assign_locked(CONVOKE, ON_PULL);

        assert_eq!(
            state.is_spell_assignable(character, TREE_OF_LIFE, AOE_80_PERCENT),
            Assignability::HasAssignedExclusives
        );
    }
}
