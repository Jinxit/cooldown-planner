use fight_domain::{AttackUuid, CharacterUuid, LookupKey, SpellUuid};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AssignmentUuid(Uuid);

impl AssignmentUuid {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for AssignmentUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.simple())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct Assignment {
    pub uuid: AssignmentUuid,
    pub character: CharacterUuid,
    pub spell: SpellUuid,
    pub attack: AttackUuid,
    pub forced: bool,
}

impl Assignment {
    pub fn new(
        character: CharacterUuid,
        spell: SpellUuid,
        attack: AttackUuid,
        forced: bool,
    ) -> Self {
        Self {
            uuid: AssignmentUuid::new(),
            character,
            spell,
            attack,
            forced,
        }
    }
}

impl LookupKey for Assignment {
    type Key = AssignmentUuid;

    fn lookup_key(&self) -> &Self::Key {
        &self.uuid
    }
}
