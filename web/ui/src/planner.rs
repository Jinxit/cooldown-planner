use serde::{Deserialize, Serialize};

use auto_battle_net::LocalizedString;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlannerRealm {
    pub name: LocalizedString,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PlannerUser {
    pub name: String,
    pub realm: PlannerRealm,
    pub guild: Option<String>,
}
