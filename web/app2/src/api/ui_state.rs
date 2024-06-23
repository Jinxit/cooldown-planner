use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use leptos::*;
use leptos::prelude::Signal;
use tracing::warn;

use fight_domain::{Attack, AttackUuid, Character, CharacterUuid, Lookup, Spell, SpellUuid};
use i18n::LocalizedString;
use optimizer::{Assignment, AssignmentState};

use crate::api::ui_character::{UiCharacter, UiCharacterTemplate};
use crate::api::ui_fight::UiFight;
use crate::components::*;
use crate::misc::flatten_ok::FlattenOk;
use crate::misc::localized_string_with_context::LocalizedStringWithContext;


#[derive(Copy, Clone)]
struct MutableUiState {
    ui_characters: RwSignal<Lookup<UiCharacter>>,
    general_character: RwSignal<UiCharacter>,
    suggested_assignments: RwSignal<Lookup<Assignment>>,
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
    all_assignments: Signal<HashMap<(CharacterUuid, AttackUuid), Lookup<Assignment>>>,
}

impl UiState {
    pub fn new() -> Self {
        let ui_state = MutableUiState {
            ui_characters: RwSignal::new(Lookup::default()),
            general_character: RwSignal::new(UiCharacter::new(
                CharacterUuid::new(),
                UiCharacterTemplate::new_general(),
                true,
            )),
            suggested_assignments: RwSignal::new(Lookup::default()),
            planning: RwSignal::new(false),
            selected_fight_index: RwSignal::new(0),
        };

        Effect::new(move |_| {
            let general_name = i18n::general().localize().to_string();
            ui_state.general_character.update(|general_character| {
                general_character.name = Some(general_name);
            });
        });

        let fights = fights::mythic_aberrus();
        let selected_fight = Signal::derive(move || {
            fights
                .get()
                .get(ui_state.selected_fight_index.get())
                .cloned()
        });
        let attacks = Signal::derive(move || {
            selected_fight
                .get()
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
                .filter(|a| a.state == AssignmentState::Locked)
                .map(|a| Assignment {
                    character: a.character,
                    spell: a.spell,
                    attack: a.attack,
                    state: a.state,
                })
                .collect()
        });
        let all_assignments = Signal::derive(move || {
            let attacks = attacks.get();
            ui_characters.with(|ui_characters| {
                let base = ui_characters
                    .iter()
                    .filter(|c| c.name.is_some())
                    .flat_map(|c| {
                        c.spells.iter().flat_map(|s| {
                            attacks.iter().map(|a| Assignment {
                                character: c.uuid,
                                spell: s.uuid,
                                attack: a.uuid,
                                state: AssignmentState::Unassigned,
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
                .map(|a| Assignment {
                    character: a.character,
                    spell: a.spell,
                    attack: a.attack,
                    state: AssignmentState::Suggested,
                })
                .collect::<Lookup<_>>(),
        )
    }

    pub fn toggle_assignment(&self, assignment: &Assignment) {
        self.ui_state.ui_characters.update(|ui_characters| {
            let ui_character = ui_characters.get_mut(&assignment.character).unwrap();
            match (
                assignment.state,
                self.is_spell_assignable(assignment.clone()),
            ) {
                (AssignmentState::Locked, _) => {
                    // remove currently forced assignment
                    ui_character.assignments.take(&(assignment.character, assignment.spell, assignment.attack));
                }
                (_, true) => {
                    // create new forced assignment
                    ui_character.assignments.put(assignment.clone());
                }
                (_, false) => {
                    // do nothing
                }
            };
        })
    }

    pub fn lock_suggestions(&self) {
        self.ui_state.ui_characters.update(|ui_characters| {
            for ui_character in ui_characters.iter_mut() {
                for assignment in ui_character.assignments.iter_mut() {
                    if assignment.state == AssignmentState::Suggested {
                        assignment.state = AssignmentState::Locked;
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
            let character = ui_characters.get_mut(&character).unwrap();
            let spell = character.spells.get_mut(&spell).unwrap();
            spell.enabled = !spell.enabled;
        })
    }

    pub fn set_selected_fight_index(&self, index: usize) {
        self.ui_state.selected_fight_index.set(index)
    }

    // TODO: consider changing this to be AsyncDerived
    pub fn is_spell_assignable(&self, assignment: Assignment) -> bool {
        let ui_characters = self.ui_characters();
        let attacks = self.attacks();
        let locked_assignments = self.locked_assignments();

        let character_uuid = assignment.character;
        let attack_uuid = assignment.attack;
        let spell_uuid = assignment.spell;

        let spells = &ui_characters.get(&character_uuid).unwrap().spells;
        let spell = spells.get(&spell_uuid).unwrap();
        let attack = &attacks.get(&attack_uuid).unwrap();

        let forced_assignments_for_character = locked_assignments
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
                let other_timer = attacks.get(&other_usage.attack).unwrap().timer.clone();
                other_timer.static_timer().abs_diff(attack_timer) <= spell.cooldown
            })
            .count();

        if other_usages_within_cooldown >= spell.charges {
            return false;
        }

        true
    }

    // TODO: consider changing the below to be AsyncDerived rather than auto-getting and defaulting
    pub fn all_assignments(&self) -> HashMap<(CharacterUuid, AttackUuid), Lookup<Assignment>> {
        self.all_assignments.get()
    }

    pub fn attacks(&self) -> Lookup<Attack> {
        self.attacks.get()
    }

    pub fn selected_fight(&self) -> Option<UiFight> {
        self.selected_fight.get()
    }

    pub fn fights(&self) -> Vec<UiFight> {
        self.fights.get()
    }

    // TODO: consider changing the below to be Signals rather than auto-getting
    pub fn locked_assignments(&self) -> Lookup<Assignment> {
        self.locked_assignments.get()
    }

    pub fn ui_character_editing(&self, uuid: CharacterUuid) -> bool {
        self.ui_characters().get(&uuid).map(|c| c.editing).unwrap_or(false)
    }

    pub fn ui_characters(&self) -> Lookup<UiCharacter> {
        self.ui_characters.get()
    }

    pub fn characters(&self) -> Lookup<Character> {
        self.characters.get()
    }

    pub fn selected_fight_index(&self) -> usize {
        self.ui_state.selected_fight_index.get()
    }

    pub fn planning(&self) -> bool {
        self.ui_state.planning.get()
    }
}
