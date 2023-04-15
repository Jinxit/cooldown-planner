use crate::serde_not_nan::{deserialize_not_nan, serialize_not_nan};
use crate::{Identifier, LookupKey, TimeStep};
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SpellUuid(Uuid);

impl SpellUuid {
    pub fn new(uuid: &'static str) -> SpellUuid {
        Self(Uuid::parse_str(uuid).unwrap())
    }
}

impl Display for SpellUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.simple())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Spell {
    pub uuid: SpellUuid,
    pub name: String,
    pub icon_text: Option<String>,
    #[serde(
        default = "default_power",
        serialize_with = "serialize_not_nan",
        deserialize_with = "deserialize_not_nan"
    )]
    pub power: NotNan<f64>,
    pub cooldown: TimeStep,
    pub cast_time: TimeStep,
    pub identifier: Identifier,
    #[serde(default = "default_charges")]
    pub charges: usize,
    #[serde(default)]
    pub exclusive_with: BTreeSet<Identifier>,
    pub enabled: bool,
    pub minor: bool,
}

impl LookupKey for Spell {
    type Key = SpellUuid;

    fn lookup_key(&self) -> &Self::Key {
        &self.uuid
    }
}

fn default_power() -> NotNan<f64> {
    NotNan::new(1.0).unwrap()
}

fn default_charges() -> usize {
    1
}
