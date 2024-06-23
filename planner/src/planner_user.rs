use serde::{Deserialize, Serialize};

use crate::planner_realm::PlannerRealm;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PlannerUser {
    pub name: String,
    pub realm: PlannerRealm,
    pub guild: Option<String>,
}
