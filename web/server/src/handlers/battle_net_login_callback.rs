use app_package::session::{BattleNetUser, CooldownPlannerSession};
use auto_battle_net::oauth::user_authentication::user_info::UserInfoRequest;
use auto_battle_net::{BattleNetClientAsync, Region, ReqwestBattleNetClient};
use axum::extract::Query;
use axum::response::{Html, IntoResponse, Response};
use axum::Extension;
use http::StatusCode;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeChallenge, RedirectUrl,
    TokenResponse, TokenUrl,
};
use paseto_sessions::Session;
use redact::Secret;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthorizationQuery {
    code: Secret<String>,
    state: String,
}

#[axum::debug_handler]
pub async fn battle_net_login_callback(
    Extension(session): Extension<Session<CooldownPlannerSession>>,
    Query(authorization): Query<AuthorizationQuery>,
) -> Response {
    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.
    let oauth_client = BasicClient::new(
        ClientId::new(std::env::var("BNET_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(
            std::env::var("BNET_CLIENT_SECRET").unwrap(),
        )),
        AuthUrl::new("https://oauth.battle.net/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth.battle.net/token".to_string()).unwrap()),
    )
    .set_redirect_uri(
        RedirectUrl::new(format!("http://localhost:3000/bnet/login-callback")).unwrap(),
    );

    // Generate a PKCE challenge.
    let pkce_verifier = PkceCodeChallenge::new_random_sha256().1;

    let return_state = session.data_mut().return_state.take();
    if let Some(return_state) = return_state {
        if return_state.state != authorization.state {
            return (StatusCode::UNAUTHORIZED, "Invalid state").into_response();
        }

        // Now you can trade it for an access token.
        let token_result = oauth_client
            .exchange_code(AuthorizationCode::new(
                authorization.code.expose_secret().clone(),
            ))
            // Set the PKCE code verifier.
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .unwrap();

        // Unwrapping token_result will either produce a Token or a RequestTokenError.
        let access_token = token_result.access_token().secret();

        let user_info = ReqwestBattleNetClient {
            region: Region::Europe,
            access_token: access_token.clone(),
        }
        .call_async(UserInfoRequest {})
        .await
        .unwrap();

        let battlenet_user = BattleNetUser {
            id: user_info.id,
            battletag: user_info.battletag,
            access_token: access_token.clone(),
        };

        session.data_mut().user = Some(battlenet_user);

        (
            StatusCode::OK,
            Html(format!(
                "<!DOCTYPE html>\
                    <html>\
                        <head>\
                            <meta http-equiv=\"refresh\" content=\"0; url='{}'\">\
                        </head>\
                        <body>\
                        </body>\
                    </html>",
                &return_state.url
            )),
        )
            .into_response()
        //Err(Redirect::temporary(path.as_str()).into_response())
    } else {
        (StatusCode::UNAUTHORIZED, "Invalid state").into_response()
    }
}
