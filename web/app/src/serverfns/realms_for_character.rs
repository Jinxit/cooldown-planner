use crate::context::{PlannerRealm, PlayerClass};
use crate::serverfns::character_summary;
use crate::session::CooldownPlannerSession;
use auto_battle_net::game_data::playable_class::playable_class::{
    PlayableClassRequest, PlayableClassResponse,
};
use auto_battle_net::game_data::realm::realms_index::{Realms, RealmsIndexRequest};
use auto_battle_net::profile::character_profile::character_profile_status::{
    CharacterProfileStatusRequest, CharacterProfileStatusResponse,
};
use auto_battle_net::profile::character_profile::character_profile_summary::CharacterProfileSummaryResponse;
use auto_battle_net::{BattleNetClientAsync, LocalizedString};
use auto_battle_net::{BattleNetError, BattleNetResult, BattleNetServerError, Region};
use base64::prelude::{BASE64_URL_SAFE, BASE64_URL_SAFE_NO_PAD};
use base64::Engine;
use bytes::Bytes;
//use cached::proc_macro::cached;
use futures_util::future::join_all;
use http::Method;
use http::StatusCode;
use leptos::use_context;
use leptos::*;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, instrument};
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RealmClassEntry {
    pub realm: PlannerRealm,
    pub class: LocalizedString,
}

#[instrument]
#[server(RealmsForCharacter, "/bnet", "GetCbor")]
pub async fn realms_for_character(
    character_name: String,
    region: Region,
) -> Result<Vec<RealmClassEntry>, ServerFnError> {
    use crate::serverfns::util::{get_bnet_client, get_session, try_fetch_cached};
    use auto_battle_net::ReqwestBattleNetClient;
    use paseto_sessions::Session;

    async fn realms(region: Region) -> Result<Vec<PlannerRealm>, ServerFnError> {
        let client = get_bnet_client(region).await?;
        let realms = client.call_async(RealmsIndexRequest {}).await?.realms;
        Ok(realms
            .into_iter()
            .map(|r| PlannerRealm {
                name: r.name.clone(),
                slug: r.slug.clone(),
            })
            .collect())
    }

    async fn playable_class(
        class_id: i64,
        region: Region,
    ) -> Result<PlayableClassResponse, ServerFnError> {
        let client = get_bnet_client(region).await?;
        let playable_class = client.call_async(PlayableClassRequest { class_id }).await?;
        Ok(playable_class)
    }

    async fn inner(
        user_id: u64,
        character_name: String,
        region: Region,
    ) -> Result<Vec<RealmClassEntry>, ServerFnError> {
        let client = get_bnet_client(region).await?;
        let realms = try_fetch_cached(&(region,), Duration::from_secs(24 * 60 * 60), move || {
            realms(region)
        })
        .await?;
        let realms_and_validity = realms.into_iter().map(move |r| {
            let character_name = character_name.clone();
            let client = client.clone();
            async move {
                let realm_slug = r.slug.clone();
                let valid = client
                    .call_async(CharacterProfileStatusRequest {
                        realm_slug: realm_slug.clone(),
                        character_name: character_name.clone(),
                    })
                    .await
                    .map(|s| s.is_valid)
                    .unwrap_or(false);
                if !valid {
                    return None;
                }

                let summary = try_fetch_cached(
                    &(character_name.clone(), realm_slug.clone(), region),
                    Duration::from_secs(24 * 60 * 60),
                    || character_summary(character_name.clone(), realm_slug.clone(), region),
                )
                .await
                .ok()?;

                let class_id = summary.character_class.id;
                let class = try_fetch_cached(
                    &(class_id, region),
                    Duration::from_secs(24 * 60 * 60),
                    move || playable_class(class_id, region),
                )
                .await
                .ok()?
                .name;

                Some(RealmClassEntry {
                    realm: PlannerRealm {
                        name: r.name,
                        slug: r.slug,
                    },
                    class,
                })
            }
        });

        let realms = join_all(realms_and_validity).await;
        let valid_realms = realms.into_iter().flatten().collect();

        Ok(valid_realms)
    }

    let user_id = get_session()
        .await
        .ok_or_else(|| ServerFnError::MissingArg("session".to_string()))?
        .data()
        .user
        .as_ref()
        .ok_or_else(|| ServerFnError::MissingArg("user".to_string()))?
        .id;

    try_fetch_cached(
        &(user_id, character_name.clone(), region),
        Duration::from_secs(24 * 60 * 60),
        move || inner(user_id, character_name, region),
    )
    .await
}
