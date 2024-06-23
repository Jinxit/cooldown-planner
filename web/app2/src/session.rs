use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct CooldownPlannerSession {
    pub user: Option<BattleNetUser>,
    pub return_state: Option<CooldownPlannerReturnState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct CooldownPlannerReturnState {
    pub url: Url,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct BattleNetUser {
    pub id: u64,
    pub battletag: String,
    pub access_token: String,
}
