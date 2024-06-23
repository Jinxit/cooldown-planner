use axum::Extension;
use leptos_axum::extract;
use server_fn::ServerFnError;

use auto_battle_net::BattleNetRequest;

use crate::session::CooldownPlannerSession;

pub async fn get_session() -> Result<paseto_sessions::Session<CooldownPlannerSession>, ServerFnError> {
    extract().await.map(|Extension(session)| session)
}
