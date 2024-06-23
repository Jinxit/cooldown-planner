use leptos::*;
use thiserror::Error;
use tracing::error;

use auto_battle_net::{
    BattleNetClientAsync, BattleNetRequest,
};
use auto_battle_net::ReqwestBattleNetClient;
use i18n::Region;

use super::get_session;

#[derive(Debug, Error)]
pub enum GetBNetClientError {
    #[error("No session cookie found or it was invalid")]
    NoSession,
    #[error("No user is logged in for the current session")]
    NoUser,
}

pub async fn get_bnet_client(
    region: Region,
) -> Result<impl BattleNetClientAsync + Clone + Send + Sync, GetBNetClientError> {
    let session = get_session().await.ok_or(GetBNetClientError::NoSession)?;
    let user = session
        .data()
        .clone()
        .user
        .ok_or(GetBNetClientError::NoUser)?;

    Ok(ReqwestBattleNetClient {
        region,
        access_token: user.access_token,
    })
}

pub async fn get_bnet_client_regionless(
) -> Result<impl BattleNetClientAsync + Clone + Send + Sync, GetBNetClientError> {
    get_bnet_client(Region::Europe).await
}
