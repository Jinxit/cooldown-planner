use crate::api::ui_spell::UiSpell;
use crate::components::specs;
use crate::context::PlannerRealm;
use crate::localization::general;
use crate::misc::localized_string_with_context::LocalizedStringWithContext;
use auto_battle_net::{Locale, LocalizedString};
use fight_domain::{AttackUuid, Character, CharacterUuid, Lookup, LookupKey, Spell, SpellUuid};
use leptos::*;
use optimizer::{Assignment, AssignmentUuid};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UiAssignmentState {
    Forced,
    Suggested,
    Unassigned,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiAssignment {
    pub uuid: AssignmentUuid,
    pub character: CharacterUuid,
    pub spell: SpellUuid,
    pub attack: AttackUuid,
    pub state: UiAssignmentState,
}

impl LookupKey for UiAssignment {
    type Key = AssignmentUuid;

    fn lookup_key(&self) -> &Self::Key {
        &self.uuid
    }
}
