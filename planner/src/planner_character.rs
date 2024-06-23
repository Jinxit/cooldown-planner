use fight_domain::{AttackUuid, Character, CharacterUuid, Lookup, LookupKey, Spell, SpellUuid};
use i18n::{Locale, LocalizedString};
use optimizer::AssignmentState;

use crate::planner_assignments::PlannerAssignments;
use crate::planner_realm::PlannerRealm;
use crate::specs;

#[derive(Debug, Clone)]
pub enum PlannerCharacterTemplate {
    Known {
        name: String,
        realm: PlannerRealm,
        class: LocalizedString,
    },
    Custom {
        name: String,
    },
    Unknown,
    General,
}

impl PlannerCharacterTemplate {
    pub fn name(&self) -> Option<LocalizedString> {
        match self {
            PlannerCharacterTemplate::Known { name, .. } => Some(LocalizedString::constant(name)),
            PlannerCharacterTemplate::Custom { name, .. } => Some(LocalizedString::constant(name)),
            PlannerCharacterTemplate::General => Some(i18n::general()),
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
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PlannerCharacter {
    pub uuid: CharacterUuid,
    pub name: Option<LocalizedString>,
    pub realm: Option<PlannerRealm>,
    pub class: Option<LocalizedString>,
    pub spec: Option<LocalizedString>,
    pub spells: Lookup<Spell>,
    pub assignments: PlannerAssignments,
}

impl PlannerCharacter {
    pub fn new(uuid: CharacterUuid, template: PlannerCharacterTemplate) -> Self {
        let mut s = Self {
            uuid,
            name: template.name(),
            realm: template.realm().map(ToOwned::to_owned),
            class: template.class().map(ToOwned::to_owned),
            spec: None,
            spells: Default::default(),
            assignments: Default::default(),
        };
        s.update_spells();
        s
    }

    pub fn assignment_state(&self, spell: SpellUuid, attack: AttackUuid) -> AssignmentState {
        self.assignments.get(spell, attack)
    }

    pub fn change_class(&mut self, class: LocalizedString) {
        self.class = Some(class);
        self.spec = None;
        self.spells = Lookup::default();
        self.assignments = PlannerAssignments::default();
    }

    pub fn change_spec(&mut self, spec: LocalizedString) {
        self.spec = Some(spec);
        self.update_spells();
        self.assignments = PlannerAssignments::default();
    }

    fn update_spells(&mut self) {
        if self.is_general() {
            self.spells = specs::general::spells();
        } else if let Some((class, spec)) = self.class.as_ref().zip(self.spec.as_ref()) {
            self.spells = specs::spells_for_spec(
                class.get(Locale::EnglishUnitedStates),
                spec.get(Locale::EnglishUnitedStates),
            );
        }
    }

    pub fn is_general(&self) -> bool {
        self.uuid == CharacterUuid::general()
    }

    pub fn spells(&self) -> &Lookup<Spell> {
        &self.spells
    }

    pub fn toggle_spell_enabled(&mut self, spell: SpellUuid) {
        let spell = self.spells.get_mut(&spell).unwrap();
        spell.enabled = !spell.enabled;
    }
}

impl From<PlannerCharacter> for Character {
    fn from(character: PlannerCharacter) -> Self {
        Character {
            uuid: character.uuid,
            name: character
                .name
                .map(|s| s.get(Locale::EnglishUnitedStates).to_owned())
                .unwrap_or_default(),
            spells: character.spells,
        }
    }
}

impl LookupKey for PlannerCharacter {
    type Key = CharacterUuid;

    fn lookup_key(&self) -> Self::Key {
        self.uuid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_known() {
        let template = PlannerCharacterTemplate::Known {
            name: "Test".to_string(),
            realm: PlannerRealm {
                name: LocalizedString::constant("Test"),
                slug: "test".to_string(),
            },
            class: LocalizedString::constant("Evoker"),
        };

        assert_eq!(template.class(), Some(&LocalizedString::constant("Evoker")));

        let uuid = CharacterUuid::new();
        let character = PlannerCharacter::new(uuid, template);
        assert_eq!(character.uuid, uuid);
        assert_eq!(character.class, Some(LocalizedString::constant("Evoker")));
        assert_eq!(
            character.spec,
            Some(LocalizedString::constant("Preservation"))
        );
        assert_eq!(character.name, Some(LocalizedString::constant("Test")));
        assert_eq!(
            character.realm,
            Some(PlannerRealm {
                name: LocalizedString::constant("Test"),
                slug: "test".to_string(),
            })
        );
    }

    #[test]
    fn template_custom() {
        let template = PlannerCharacterTemplate::Custom {
            name: "Test".to_string(),
        };

        let uuid = CharacterUuid::new();
        let character = PlannerCharacter::new(uuid, template);
        assert_eq!(character.uuid, uuid);
        assert_eq!(character.class, None);
        assert_eq!(character.spec, None);
        assert_eq!(character.name, Some(LocalizedString::constant("Test")));
        assert_eq!(character.realm, None);
    }

    #[test]
    fn template_general() {
        let template = PlannerCharacterTemplate::General;

        assert_eq!(template.class(), None);
        assert_eq!(template.spec(), None);

        let uuid = CharacterUuid::general();
        let character = PlannerCharacter::new(uuid, template);
        assert_eq!(character.uuid, uuid);
        assert_eq!(character.class, None);
        assert_eq!(character.spec, None);
        assert_eq!(character.name, Some(i18n::general()));
        assert_eq!(character.realm, None);
    }

    #[test]
    fn template_unknown() {
        let template = PlannerCharacterTemplate::Unknown;

        assert_eq!(template.class(), None);
        assert_eq!(template.spec(), None);

        let uuid = CharacterUuid::new();
        let character = PlannerCharacter::new(uuid, template);
        assert_eq!(character.uuid, uuid);
        assert_eq!(character.class, None);
        assert_eq!(character.spec, None);
        assert_eq!(character.name, None);
        assert_eq!(character.realm, None);
    }

    #[test]
    fn change_class_resets_spec_and_spells_and_resets_assignments() {
        let mut character = PlannerCharacter::new(CharacterUuid::new(), PlannerCharacterTemplate::Known {
            name: "Test".to_string(),
            realm: PlannerRealm {
                name: LocalizedString::constant("Test"),
                slug: "test".to_string(),
            },
            class: LocalizedString::constant("Evoker"),
        });
        character.change_spec(LocalizedString::constant("Preservation"));

        character.change_class(LocalizedString::constant("Druid"));
        assert_eq!(character.class, Some(LocalizedString::constant("Druid")));
        assert_eq!(character.spec, None);
        assert!(character.spells.is_empty());
        assert!(character.assignments.is_empty());
    }

    #[test]
    fn change_class_and_spec_updates_spells_and_resets_assignments() {
        let mut character = PlannerCharacter::new(CharacterUuid::new(), PlannerCharacterTemplate::Known {
            name: "Test".to_string(),
            realm: PlannerRealm {
                name: LocalizedString::constant("Test"),
                slug: "test".to_string(),
            },
            class: LocalizedString::constant("Evoker"),
        });
        character.change_spec(LocalizedString::constant("Preservation"));

        character.change_class(LocalizedString::constant("Druid"));
        assert!(character.spells.is_empty());

        character.change_spec(LocalizedString::constant("Restoration"));
        assert!(!character.spells.is_empty());
        assert_eq!(character.class, Some(LocalizedString::constant("Druid")));
        assert_eq!(character.spec, Some(LocalizedString::constant("Restoration")));
        assert!(character.assignments.is_empty());
    }

    #[test]
    fn change_spec_updates_spells_and_resets_assignments() {
        let mut character = PlannerCharacter::new(CharacterUuid::new(), PlannerCharacterTemplate::Known {
            name: "Test".to_string(),
            realm: PlannerRealm {
                name: LocalizedString::constant("Test"),
                slug: "test".to_string(),
            },
            class: LocalizedString::constant("Evoker"),
        });
        character.change_spec(LocalizedString::constant("Devastation"));
        let spells_devastation = character.spells.len();

        character.change_spec(LocalizedString::constant("Preservation"));
        let spells_preservation = character.spells.len();

        assert!(spells_preservation > spells_devastation);
        assert_eq!(character.class, Some(LocalizedString::constant("Evoker")));
        assert_eq!(character.spec, Some(LocalizedString::constant("Preservation")));
        assert!(character.assignments.is_empty());
    }
}
