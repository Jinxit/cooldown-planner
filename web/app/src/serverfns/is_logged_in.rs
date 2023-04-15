use auto_battle_net::BattleNetClientAsync;
use auto_battle_net::{oauth::user_authentication::user_info::UserInfoRequest, Region};
use leptos::{server, ServerFnError};
use thiserror::Error;
use tracing::instrument;

#[instrument]
#[server(IsLoggedIn, "/bnet")]
pub async fn is_logged_in() -> Result<bool, ServerFnError> {
    use super::util::{get_bnet_client, get_bnet_client_regionless, get_session};
    use auto_battle_net::ReqwestBattleNetClient;

    let client = get_bnet_client_regionless().await?; // TODO: does any region work?
    let response = client.call_async(UserInfoRequest::default()).await?;
    Ok(true)
}
