use redact::Secret;
use serde::Deserialize;

#[cfg(feature = "axum")]
pub mod axum;

#[derive(Clone, Deserialize)]
pub struct OAuthToken {
    access_token: Secret<String>,
}

impl OAuthToken {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token: Secret::new(access_token),
        }
    }

    pub fn expose_secret(&self) -> &str {
        self.access_token.expose_secret()
    }
}
