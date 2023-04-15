use super::get_session;
use crate::session::CooldownPlannerSession;
use auto_battle_net::ReqwestBattleNetClient;
use auto_battle_net::{
    BattleNetClientAsync, BattleNetRequest, BattleNetResult, BattleNetServerError, Region,
};
use base64::prelude::{BASE64_URL_SAFE, BASE64_URL_SAFE_NO_PAD};
use base64::Engine;
use bytes::Bytes;
use http::StatusCode;
use leptos::*;
use reqwest::Response;
use serde_lite::{Deserialize, Serialize};
use std::num::NonZeroU16;
use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tracing::{error, instrument};
use url::Url;

#[derive(Debug, Error)]
pub enum GetBNetClientError {
    #[error("No session cookie found or it was invalid")]
    NoSession,
    #[error("No user is logged in for the current session")]
    NoUser,
}

pub async fn get_bnet_client(
    region: Region,
) -> Result<impl BattleNetClientAsync + Clone, GetBNetClientError> {
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
) -> Result<impl BattleNetClientAsync + Clone, GetBNetClientError> {
    get_bnet_client(Region::Europe).await
}
