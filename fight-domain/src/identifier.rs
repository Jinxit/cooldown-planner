use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub enum Identifier {
    Spell(u32),
    Icon(String, u32),
    Marker(RaidMarker),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub enum RaidMarker {
    Star = 1,
    Circle = 2,
    Diamond = 3,
    Triangle = 4,
    Square = 5,
    Moon = 6,
    Cross = 7,
    Skull = 8,
}
