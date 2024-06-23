use redact::Secret;
use serde::Deserialize;
use auto_battle_net::BattleNetAccessToken;

#[cfg(feature = "axum")]
pub mod axum;

#[derive(Clone, Deserialize)]
pub struct OAuthToken {
    access_token: Secret<BattleNetAccessToken>,
}

impl OAuthToken {
    pub fn new(access_token: BattleNetAccessToken) -> Self {
        Self {
            access_token: Secret::new(access_token),
        }
    }

    pub fn expose_secret(&self) -> &BattleNetAccessToken {
        self.access_token.expose_secret()
    }
}
