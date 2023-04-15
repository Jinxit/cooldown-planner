use crate::session::CooldownPlannerSession;
use auto_battle_net::{
    BattleNetClientAsync, BattleNetRequest, BattleNetResult, BattleNetServerError, Region,
};
use axum::Extension;
use base64::prelude::{BASE64_URL_SAFE, BASE64_URL_SAFE_NO_PAD};
use base64::Engine;
use bytes::Bytes;
use http::StatusCode;
use leptos::*;
use leptos_axum::extract;
use reqwest::Response;
use serde_lite::{Deserialize, Serialize};
use std::num::NonZeroU16;
use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, instrument};
use url::Url;

pub async fn get_session() -> Option<paseto_sessions::Session<CooldownPlannerSession>> {
    extract(
        move |Extension(session): Extension<paseto_sessions::Session<CooldownPlannerSession>>| async move {
            session
        },
    )
    .await
    .ok()
}
