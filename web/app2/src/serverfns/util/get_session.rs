use axum::Extension;
use leptos_axum::extract;

use auto_battle_net::BattleNetRequest;

use crate::session::CooldownPlannerSession;

pub async fn get_session() -> Option<paseto_sessions::Session<CooldownPlannerSession>> {
    extract().await.ok().map(|Extension(session)| session)
}
