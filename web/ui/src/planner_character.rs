use auto_battle_net::{Locale, LocalizedString};
use fight_domain::{Character, CharacterUuid, Lookup, LookupKey, Spell};
use optimizer::Assignment;

use crate::planner::PlannerRealm;
use crate::specs;

pub enum PlannerCharacterTemplate {
    Known {
        name: String,
        realm: PlannerRealm,
        class: LocalizedString,
        spec: LocalizedString,
    },
    Custom {
        name: String,
    },
    Unknown,
    General,
}

impl PlannerCharacterTemplate {
    pub fn name(&self) -> Option<&str> {
        match self {
            PlannerCharacterTemplate::Known { name, .. } => Some(name),
            PlannerCharacterTemplate::Custom { name, .. } => Some(name),
            _ => None,
        }
    }

    pub fn realm(&self) -> Option<&PlannerRealm> {
        match self {
            PlannerCharacterTemplate::Known { realm, .. } => Some(realm),
            _ => None,
        }
    }

    pub fn class(&self) -> Option<&LocalizedString> {
        match self {
            PlannerCharacterTemplate::Known { class, .. } => Some(class),
            _ => None,
        }
    }

    pub fn spec(&self) -> Option<&LocalizedString> {
        match self {
            PlannerCharacterTemplate::Known { spec, .. } => Some(spec),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PlannerCharacter {
    pub uuid: CharacterUuid,
    pub name: Option<String>,
    pub realm: Option<PlannerRealm>,
    pub class: Option<LocalizedString>,
    pub spec: Option<LocalizedString>,
    pub spells: Lookup<Spell>,
    pub assignments: Lookup<Assignment>,
    pub suggested_assignments: Lookup<Assignment>,
}

impl PlannerCharacter {
    pub fn new(uuid: CharacterUuid, template: PlannerCharacterTemplate) -> Self {
        match template {
            PlannerCharacterTemplate::General => Self {
                uuid,
                name: None,
                realm: None,
                class: None,
                spec: None,
                spells: specs::general::spells(),
                assignments: Lookup::default(),
            },
            PlannerCharacterTemplate::Known { class, spec, .. } => Self {
                uuid,
                name: None,
                realm: None,
                class: Some(class.clone()),
                spec: Some(spec.clone()),
                spells: specs::spells_for_spec(
                    class.get(Locale::EnglishUnitedStates),
                    spec.get(Locale::EnglishUnitedStates),
                ),
                assignments: Lookup::default(),
            },
            _ => Self {
                uuid,
                name: None,
                realm: None,
                class: None,
                spec: None,
                spells: Lookup::default(),
                assignments: Lookup::default(),
            },
        }
    }

    pub fn as_optimizer_character(&self) -> Character {
        Character {
            uuid: self.uuid,
            name: self.name.clone().unwrap_or_default(),
            spells: self.spells.clone(),
        }
    }
}

impl LookupKey for PlannerCharacter {
    type Key = CharacterUuid;

    fn lookup_key(&self) -> &Self::Key {
        &self.uuid
    }
}
