use leptos::prelude::*;
use tracing::instrument;
use url::Url;

#[instrument]
#[server(prefix = "/bnet")]
pub async fn battle_net_login_url(return_url: Url) -> Result<Option<Url>, ServerFnError> {
    use crate::serverfns::util::get_session;
    use crate::session::{CooldownPlannerReturnState, CooldownPlannerSession};
    use oauth2::basic::BasicClient;
    use oauth2::{
        AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenUrl,
    };
    use paseto_sessions::Session;

    let session = get_session()
        .await
        .expect("all requests should have a session");

    if session.data().user.is_some() {
        return Ok(None);
    }

    let oauth_client = BasicClient::new(
        ClientId::new(std::env::var("BNET_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(
            std::env::var("BNET_CLIENT_SECRET").unwrap(),
        )),
        AuthUrl::new("https://oauth.battle.net/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth.battle.net/token".to_string()).unwrap()),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:3000/bnet/login-callback".to_string()).unwrap(),
    );

    // Generate a PKCE challenge.
    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(oauth2::Scope::new("openid".to_string()))
        .add_scope(oauth2::Scope::new("wow.profile".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    session.data_mut().return_state = Some(CooldownPlannerReturnState {
        url: return_url,
        state: csrf_token.secret().clone(),
    });

    Ok(Some(auth_url))
}
