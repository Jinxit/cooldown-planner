use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum BattleNetAccessToken {
    ClientCredentials(String),
    AuthorizationCode(String),
}

impl Display for BattleNetAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BattleNetAccessToken::ClientCredentials(token) => write!(f, "{token}"),
            BattleNetAccessToken::AuthorizationCode(token) => write!(f, "{token}"),
        }
    }
}