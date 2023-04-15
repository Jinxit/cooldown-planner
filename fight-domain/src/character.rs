use crate::{Lookup, LookupKey, Spell};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CharacterUuid(Uuid);

impl CharacterUuid {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for CharacterUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.simple())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Character {
    pub uuid: CharacterUuid,
    pub name: String,
    pub spells: Lookup<Spell>,
}

impl LookupKey for Character {
    type Key = CharacterUuid;

    fn lookup_key(&self) -> &Self::Key {
        &self.uuid
    }
}
