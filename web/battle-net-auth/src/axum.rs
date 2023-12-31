use crate::OAuthToken;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::Extension;
//use cached::proc_macro::cached;
use redact::Secret;
use std::convert::Infallible;

#[derive(Clone, Debug)]
struct OAuthCredentials {
    client_id: Secret<String>,
    client_secret: Secret<String>,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for OAuthToken
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let oauth_credentials = req.extensions.get::<OAuthCredentials>().unwrap();
        let oauth_token = bnet_access_token(
            oauth_credentials.client_id.expose_secret().to_string(),
            oauth_credentials.client_secret.expose_secret().to_string(),
        )
        .await;
        Ok(oauth_token)
    }
}

pub trait OAuthTokenExt {
    fn with_battle_net_auth(self, client_id: String, client_secret: String) -> Self;
}

impl<S> OAuthTokenExt for axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_battle_net_auth(self, client_id: String, client_secret: String) -> Self {
        self.layer(Extension(OAuthCredentials {
            client_id: Secret::new(client_id),
            client_secret: Secret::new(client_secret),
        }))
    }
}

//#[cached(time = 3600)]
async fn bnet_access_token(client_id: String, client_secret: String) -> OAuthToken {
    let access_token: OAuthToken = {
        reqwest::Client::new()
            .post("https://oauth.battle.net/token?grant_type=client_credentials")
            .basic_auth(client_id, Some(client_secret))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    };
    access_token
}
