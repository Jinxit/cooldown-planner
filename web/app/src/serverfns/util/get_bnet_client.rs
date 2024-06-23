use async_trait::async_trait;
use leptos::prelude::*;
use leptos_axum::extract;
use thiserror::Error;
use tracing::error;
use battle_net_auth::OAuthToken;

use auto_battle_net::{
    BattleNetAccessToken, BattleNetClientAsync, BattleNetRequest, BattleNetResult,
};
use auto_battle_net::ReqwestBattleNetClient;
use i18n::Region;

use super::get_session;

#[derive(Debug, Error)]
pub enum GetBNetClientError {
    #[error("No session cookie found or it was invalid: {0}")]
    NoSession(ServerFnError),
    #[error("No user is logged in for the current session")]
    NoUser,
    #[error("No server token found or it was invalid: {0}")]
    NoServerToken(ServerFnError),
}

pub enum ClientType {
    UserOnly,
    ServerOnly,
    AllowFallback,
}

pub async fn get_bnet_client(
    region: Region,
    client_type: ClientType,
) -> Result<impl BattleNetClientAsync + Clone + Send + Sync, GetBNetClientError> {
    let access_token = get_access_token(client_type).await?;

    Ok(ReqwestBattleNetClient {
        region,
        access_token,
    })
}

pub async fn get_bnet_client_regionless(
    client_type: ClientType,
) -> Result<impl BattleNetClientAsync + Clone + Send + Sync, GetBNetClientError> {
    get_bnet_client(Region::Europe, client_type).await
}

async fn get_access_token(client_type: ClientType) -> Result<BattleNetAccessToken, GetBNetClientError> {
    let server_token = extract::<OAuthToken>().await.map(|s| s.expose_secret().clone()).map_err(|e| GetBNetClientError::NoServerToken(e));
    let user_token = get_session()
        .await
        .map_err(|e| GetBNetClientError::NoSession(e))
        .and_then(|s| s.data().user.clone().ok_or(GetBNetClientError::NoUser))
        .map(|u| u.access_token);

    match client_type {
        ClientType::ServerOnly => server_token,
        ClientType::UserOnly => user_token,
        ClientType::AllowFallback => user_token.or(server_token),
    }
}
