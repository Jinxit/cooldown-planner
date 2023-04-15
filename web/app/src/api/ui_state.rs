use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::api::ui_assignment::UiAssignmentState;
use crate::api::ui_character::{UiCharacter, UiCharacterTemplate};
use crate::api::ui_fight::UiFight;
use crate::api::use_optimizer;
use crate::{components::*, localization};
use crate::localization::{encounter, preferences};
use crate::misc::flatten_ok::FlattenOk;
use crate::misc::localized_string_with_context::LocalizedStringWithContext;
use crate::reactive::map::Map;
use auto_battle_net::LocalizedString;
use fight_domain::{
    Attack, AttackUuid, Character, CharacterUuid, Lookup, LookupKey, Spell, SpellUuid,
};
use itertools::Itertools;
use leptos::*;
use optimizer::{Assignment, AssignmentUuid};

use super::ui_assignment::UiAssignment;
use super::ui_spell::UiSpell;

#[derive(Copy, Clone)]
struct MutableUiState {
    ui_characters: RwSignal<Lookup<UiCharacter>>,
    general_character: RwSignal<UiCharacter>,
    suggested_assignments: RwSignal<Lookup<UiAssignment>>,
    planning: RwSignal<bool>,
    selected_fight_index: RwSignal<usize>,
}

#[derive(Copy, Clone)]
pub struct UiState {
    ui_state: MutableUiState,
    fights: Signal<Vec<UiFight>>,
    selected_fight: Signal<Option<UiFight>>,
    attacks: Signal<Lookup<Attack>>,
    ui_characters: Signal<Lookup<UiCharacter>>,
    characters: Signal<Lookup<Character>>,
    locked_assignments: Signal<Lookup<Assignment>>,
    all_assignments: Signal<HashMap<(CharacterUuid, AttackUuid), Lookup<UiAssignment>>>,
}

impl UiState {
    pub fn new() -> Self {
        let ui_state = MutableUiState {
            ui_characters: create_rw_signal(Lookup::default()),
            general_character: create_rw_signal(UiCharacter::new(
                CharacterUuid::new(),
                UiCharacterTemplate::new_general(),
                true,
            )),
            suggested_assignments: create_rw_signal(Lookup::default()),
            planning: create_rw_signal(false),
            selected_fight_index: create_rw_signal(0),
        };

        create_effect(move |_| {
            let general_name = localization::general().localize().to_string();
            ui_state.general_character.update(|general_character| {
                general_character.name = Some(general_name);
            });
        });

        let fights = fights::mythic_aberrus();
        let selected_fight =
            Signal::derive(move || fights().get(ui_state.selected_fight_index.get()).cloned());
        let attacks = Signal::derive(move || {
            selected_fight()
                .map(|f| f.attacks.get().clone())
                .unwrap_or_default()
        });
        let ui_characters = Signal::derive(move || {
            let mut ui_characters = ui_state.ui_characters.get();
            ui_characters.put(ui_state.general_character.get());
            ui_characters
        });
        let characters = Signal::derive(move || {
            ui_characters.with(move |ui_characters| {
                ui_characters
                    .iter()
                    .filter_map(|c| {
                        c.name.clone().map(|name| Character {
                            uuid: c.uuid,
                            name,
                            spells: c
                                .spells
                                .iter()
                                .map(|ui_spell| Spell {
                                    name: ui_spell.name.clone(),
                                    icon_text: ui_spell.icon_text.clone(),
                                    identifier: ui_spell.identifier.clone(),
                                    power: ui_spell.power,
                                    charges: ui_spell.charges,
                                    cooldown: ui_spell.cooldown,
                                    cast_time: ui_spell.cast_time,
                                    exclusive_with: ui_spell.exclusive_with.clone(),
                                    uuid: ui_spell.uuid,
                                    enabled: ui_spell.enabled,
                                    minor: ui_spell.minor,
                                })
                                .collect(),
                        })
                    })
                    .collect()
            })
        });
        let locked_assignments = Signal::derive(move || {
            ui_characters()
                .into_iter()
                .filter(|c| c.name.is_some())
                .flat_map(|c| c.assignments.into_iter())
                .filter(|a| a.state == UiAssignmentState::Forced)
                .map(|a| Assignment {
                    uuid: a.uuid,
                    character: a.character,
                    spell: a.spell,
                    attack: a.attack,
                    forced: true,
                })
                .collect()
        });
        let all_assignments = Signal::derive(move || {
            ui_characters.with(|ui_characters| {
                let attacks = attacks();
                let base = ui_characters
                    .iter()
                    .filter(|c| c.name.is_some())
                    .flat_map(|c| {
                        c.spells.iter().flat_map(|s| {
                            attacks.iter().map(|a| UiAssignment {
                                uuid: AssignmentUuid::new(),
                                character: c.uuid,
                                spell: s.uuid,
                                attack: a.uuid,
                                state: UiAssignmentState::Unassigned,
                            })
                        })
                    });
                let suggested = ui_state.suggested_assignments.get().into_iter();
                let locked = ui_characters
                    .iter()
                    .filter(|c| c.name.is_some())
                    .flat_map(|c| c.assignments.iter())
                    .cloned();

                base.chain(suggested)
                    .chain(locked)
                    .map(|a| ((a.character, a.attack), a))
                    .into_grouping_map()
                    .collect()
            })
        });

        Self {
            ui_state,
            fights,
            selected_fight,
            attacks,
            ui_characters,
            characters,
            locked_assignments,
            all_assignments,
        }
    }

    pub fn add_ui_character(&self, new_character: UiCharacterTemplate) {
        self.ui_state.ui_characters.update(|ui_characters| {
            ui_characters.put(UiCharacter::new(CharacterUuid::new(), new_character, false))
        });
    }

    pub fn replace_ui_character(
        &self,
        prev_uuid: CharacterUuid,
        new_character: UiCharacterTemplate,
    ) {
        let character = UiCharacter::new(CharacterUuid::new(), new_character, false);
        self.ui_state
            .ui_characters
            .update(|ui_characters| ui_characters.replace(&prev_uuid, character));
    }

    pub fn remove_ui_character(&self, uuid: CharacterUuid) {
        self.ui_state.ui_characters.update(|ui_characters| {
            ui_characters.take(&uuid);
        });
    }

    pub fn set_planning(&self, value: bool) {
        self.ui_state.planning.set(value);
    }

    pub fn update_assignment_suggestions(&self, suggestions: Lookup<Assignment>) {
        self.ui_state.suggested_assignments.set(
            suggestions
                .into_iter()
                .map(|a| UiAssignment {
                    uuid: a.uuid,
                    character: a.character,
                    spell: a.spell,
                    attack: a.attack,
                    state: UiAssignmentState::Suggested,
                })
                .collect(),
        )
    }

    pub fn toggle_assignment(&self, assignment: &UiAssignment) {
        self.ui_state.ui_characters.update(|ui_characters| {
            let ui_character = ui_characters.get_mut(&assignment.character).unwrap();
            match (assignment.state, self.is_spell_assignable(assignment)) {
                (UiAssignmentState::Forced, _) => {
                    // remove currently forced assignment
                    ui_character.assignments.take(&assignment.uuid);
                }
                (_, true) => {
                    // create new forced assignment
                    ui_character.assignments.put(assignment.clone());
                }
                (current, false) => {
                    // do nothing
                }
            };
        })
    }

    pub fn is_spell_assignable(&self, assignment: &UiAssignment) -> bool {
        let character_uuid = assignment.character;
        let attack_uuid = assignment.attack;
        let spell_uuid = assignment.spell;

        let ui_characters = self.ui_characters();
        let spells = &ui_characters.get(&character_uuid).unwrap().spells;
        let spell = spells.get(&spell_uuid).unwrap();
        let attacks = self.attacks();
        let attack = &attacks.get(&attack_uuid).unwrap();

        let forced_assignments_for_character = self
            .locked_assignments()
            .into_iter()
            .filter(|a| {
                a.character != character_uuid && a.attack != attack_uuid && a.spell != spell_uuid
            })
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
                let other_timer = self
                    .attacks
                    .get()
                    .get(&other_usage.attack)
                    .unwrap()
                    .timer
                    .clone();
                other_timer.static_timer().abs_diff(attack_timer) <= spell.cooldown
            })
            .count();

        if other_usages_within_cooldown >= spell.charges {
            return false;
        }

        true
    }

    pub fn lock_suggestions(&self) {
        self.ui_state.ui_characters.update(|ui_characters| {
            for ui_character in ui_characters.iter_mut() {
                for assignment in ui_character.assignments.iter_mut() {
                    if assignment.state == UiAssignmentState::Suggested {
                        assignment.state = UiAssignmentState::Forced;
                    }
                }
            }
        })
    }

    pub fn set_ui_character_editing(&self, uuid: CharacterUuid, editing: bool) {
        self.ui_state.ui_characters.update(|ui_characters| {
            ui_characters.get_mut(&uuid).unwrap().editing = editing;
        })
    }

    pub fn set_ui_character_class(&self, uuid: CharacterUuid, class: Option<LocalizedString>) {
        self.ui_state.ui_characters.update(|ui_characters| {
            ui_characters.get_mut(&uuid).unwrap().class = class;
        })
    }

    pub fn set_ui_character_spec(&self, uuid: CharacterUuid, spec: Option<LocalizedString>) {
        self.ui_state.ui_characters.update(|ui_characters| {
            ui_characters.get_mut(&uuid).unwrap().spec = spec;
        })
    }

    pub fn toggle_spell_enabled(&self, character: CharacterUuid, spell: SpellUuid) {
        self.ui_state.ui_characters.update(|ui_characters| {
            let spell = ui_characters
                .get_mut(&character)
                .unwrap()
                .spells
                .get_mut(&spell)
                .unwrap();
            spell.enabled = !spell.enabled;
        })
    }

    pub fn set_selected_fight_index(&self, index: usize) {
        self.ui_state.selected_fight_index.set(index)
    }

    pub fn ui_character_editing(&self, uuid: CharacterUuid) -> bool {
        self.ui_characters().get(&uuid).unwrap().editing
    }

    pub fn locked_assignments(&self) -> Lookup<Assignment> {
        self.locked_assignments.get()
    }

    pub fn all_assignments(&self) -> HashMap<(CharacterUuid, AttackUuid), Lookup<UiAssignment>> {
        self.all_assignments.get()
    }

    pub fn attacks(&self) -> Lookup<Attack> {
        self.attacks.get()
    }

    pub fn ui_characters(&self) -> Lookup<UiCharacter> {
        self.ui_characters.get()
    }

    pub fn characters(&self) -> Lookup<Character> {
        self.characters.get()
    }

    pub fn selected_fight(&self) -> Option<UiFight> {
        self.selected_fight.get()
    }

    pub fn selected_fight_index(&self) -> usize {
        self.ui_state.selected_fight_index.get()
    }

    pub fn fights(&self) -> Vec<UiFight> {
        self.fights.get()
    }

    pub fn planning(&self) -> bool {
        self.ui_state.planning.get()
    }
}
