use fight_domain::{AttackUuid, CharacterUuid, LookupKey, SpellUuid};
use optimizer::{AssignmentState, AssignmentUuid};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiAssignment {
    pub uuid: AssignmentUuid,
    pub character: CharacterUuid,
    pub spell: SpellUuid,
    pub attack: AttackUuid,
    pub state: AssignmentState,
}

impl LookupKey for UiAssignment {
    type Key = AssignmentUuid;

    fn lookup_key(&self) -> &Self::Key {
        &self.uuid
    }
}
