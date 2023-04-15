use crate::api::ui_assignment::{UiAssignment, UiAssignmentState};
use crate::api::ui_spell::UiSpell;
use crate::components::specs;
use crate::components::specs::general;
use crate::context::PlannerRealm;
use crate::localization::general;
use crate::misc::localized_string_with_context::LocalizedStringWithContext;
use crate::reactive::memo::Memoize;
use auto_battle_net::{Locale, LocalizedString};
use fight_domain::{Attack, Character, CharacterUuid, Lookup, LookupKey, Spell, SpellUuid};
use itertools::Itertools;
use leptos::*;
use optimizer::{Assignment, AssignmentUuid};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UiCharacterTemplate {
    name: Option<String>,
    realm: Option<PlannerRealm>,
    class: Option<LocalizedString>,
    spec: Option<LocalizedString>,
    editable: bool,
    editing: bool,
}

impl UiCharacterTemplate {
    pub fn new_known(
        name: String,
        realm: PlannerRealm,
        class: LocalizedString,
        spec: LocalizedString,
    ) -> Self {
        Self {
            name: Some(name),
            realm: Some(realm),
            class: Some(class),
            spec: Some(spec),
            editable: true,
            editing: false,
        }
    }

    pub fn new_custom(name: String) -> Self {
        Self {
            name: Some(name),
            realm: None,
            class: None,
            spec: None,
            editable: true,
            editing: false,
        }
    }

    pub fn new_unknown() -> Self {
        Self {
            name: None,
            realm: None,
            class: None,
            spec: None,
            editable: true,
            editing: true,
        }
    }

    pub fn new_general() -> Self {
        Self {
            name: None,
            realm: None,
            class: None,
            spec: None,
            editable: false,
            editing: false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct UiCharacter {
    pub uuid: CharacterUuid,
    pub name: Option<String>,
    pub realm: Option<PlannerRealm>,
    pub class: Option<LocalizedString>,
    pub spec: Option<LocalizedString>,
    pub spells: Lookup<UiSpell>,
    pub editable: bool,
    pub editing: bool,
    pub assignments: Lookup<UiAssignment>,
    pub is_general: bool,
}

impl UiCharacter {
    pub fn new(uuid: CharacterUuid, template: UiCharacterTemplate, is_general: bool) -> Self {
        let class = template.class;
        let spec = template.spec;
        let name = template.name;
        let spells = if let Some((class, spec)) = class.as_ref().zip(spec.as_ref()) {
            specs::spells_for_spec(
                class.get(Locale::EnglishUnitedStates),
                spec.get(Locale::EnglishUnitedStates),
            )
        } else if name.is_some() {
            general::spells()
        } else {
            Lookup::default()
        };

        let assignments: Lookup<UiAssignment> = Lookup::default();

        Self {
            uuid,
            name,
            realm: template.realm,
            class,
            spec,
            spells,
            editable: template.editable,
            editing: template.editing,
            assignments,
            is_general,
        }
    }
}

impl LookupKey for UiCharacter {
    type Key = CharacterUuid;

    fn lookup_key(&self) -> &Self::Key {
        &self.uuid
    }
}
