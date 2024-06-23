use serde::{Deserialize, Serialize};

use fight_domain::{AttackUuid, CharacterUuid, LookupKey, SpellUuid};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum AssignmentState {
    Locked,
    Suggested,
    Unassigned,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct Assignment {
    pub character: CharacterUuid,
    pub spell: SpellUuid,
    pub attack: AttackUuid,
    pub state: AssignmentState,
}

impl Assignment {
    pub fn new(
        character: CharacterUuid,
        spell: SpellUuid,
        attack: AttackUuid,
        state: AssignmentState,
    ) -> Self {
        Self {
            character,
            spell,
            attack,
            state,
        }
    }
}

impl LookupKey for Assignment {
    type Key = String;

    fn lookup_key(&self) -> Self::Key {
        format!("{} {} {}", self.character, self.spell, self.attack)
    }
}
