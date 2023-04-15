use crate::attack_timer::AttackTimer;
use crate::serde_not_nan::{deserialize_not_nan, serialize_not_nan};
use crate::LookupKey;
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttackUuid(Uuid);

impl AttackUuid {
    pub fn new(uuid: &'static str) -> AttackUuid {
        Self(Uuid::parse_str(uuid).unwrap())
    }
}

impl Display for AttackUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.simple())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attack {
    pub uuid: AttackUuid,
    pub name: String,
    #[serde(
        default = "default_power",
        serialize_with = "serialize_not_nan",
        deserialize_with = "deserialize_not_nan"
    )]
    pub power: NotNan<f64>,
    pub r#type: AttackType,
    pub timer: AttackTimer,
}

impl LookupKey for Attack {
    type Key = AttackUuid;

    fn lookup_key(&self) -> &Self::Key {
        &self.uuid
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AttackType {
    RaidDamage,
    RaidDamageStacked,
    RotDamage,
    Movement,
    Dispels,
    Debuffs,
    Adds,
    Generic,
}

impl TryFrom<&str> for AttackType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use AttackType::*;
        let power_mapping: HashMap<&str, AttackType> = [
            // TODO: this will change and be "much more clear"
            //       in the next version of the spreadsheet
            //       spoiler: it wasn't
            ("Red", RaidDamage),           // Raid damage
            ("Orange", RaidDamageStacked), // Raid damage, stacked raid
            ("Yellow", RotDamage),         // Rot damage
            ("Green", Movement),           // Movement
            ("Blue", Dispels),             // Dispels
            ("Purple", Debuffs),           // Debuff related
            ("Pink", Adds),                // Adds?
            ("Light Grey", Generic),       // Generic
        ]
        .into_iter()
        .collect();

        power_mapping.get(value).copied().ok_or(())
    }
}

fn default_power() -> NotNan<f64> {
    NotNan::new(1.0).unwrap()
}
