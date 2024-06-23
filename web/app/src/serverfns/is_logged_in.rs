use leptos::prelude::*;
use leptos::server_fn::codec::{Cbor, GetUrl, Json};
use thiserror::Error;
use tracing::instrument;

use auto_battle_net::BattleNetClientAsync;
use auto_battle_net::oauth::user_authentication::user_info::UserInfoRequest;

#[instrument]
#[server(prefix = "/bnet")]
pub async fn is_logged_in() -> Result<bool, ServerFnError> {
    use super::util::{get_bnet_client, get_bnet_client_regionless, get_session, ClientType};
    use auto_battle_net::ReqwestBattleNetClient;

    let client = get_bnet_client_regionless(ClientType::UserOnly).await?; // TODO: does any region work?
    let response = client.call_async(UserInfoRequest::default()).await?;
    Ok(true)
}
