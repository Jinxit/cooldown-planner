use std::num::NonZeroU16;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use base64::Engine;
use base64::prelude::{BASE64_URL_SAFE, BASE64_URL_SAFE_NO_PAD};
use bytes::Bytes;
//use cached::proc_macro::cached;
use futures_util::future::join_all;
use http::Method;
use http::StatusCode;
use leptos::prelude::*;
use leptos::server_fn::codec::{Cbor, GetUrl, Json};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use server_fn::error::NoCustomError;
use tracing::{error, instrument};
use url::Url;

use auto_battle_net::{BattleNetError, BattleNetResult, BattleNetServerError};
use auto_battle_net::BattleNetClientAsync;
use auto_battle_net::game_data::playable_class::playable_class::{
    PlayableClassRequest, PlayableClassResponse,
};
use auto_battle_net::game_data::realm::realms_index::{Realms, RealmsIndexRequest};
use auto_battle_net::profile::character_profile::character_profile_status::{
    CharacterProfileStatusRequest, CharacterProfileStatusResponse,
};
use auto_battle_net::profile::character_profile::character_profile_summary::CharacterProfileSummaryResponse;
use i18n::{LocalizedString, Region};
use planner::PlannerRealm;

use crate::serverfns::character_summary;
use crate::session::CooldownPlannerSession;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RealmClassEntry {
    pub realm: PlannerRealm,
    pub class: LocalizedString,
}

#[instrument]
#[server(prefix = "/bnet", input = GetUrl, output = Json)]
pub async fn realms_for_character(
    character_name: String,
    region: Region,
) -> Result<Vec<RealmClassEntry>, ServerFnError> {
    use crate::serverfns::util::{get_bnet_client, get_session, get_storage, ClientType};
    use auto_battle_net::ReqwestBattleNetClient;
    use paseto_sessions::Session;
    use storage::Storage;

    async fn realms(region: Region) -> Result<Vec<PlannerRealm>, ServerFnError> {
        let client = get_bnet_client(region, ClientType::AllowFallback).await?;
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
        storage: Storage,
        client: impl BattleNetClientAsync + Clone + Send + Sync,
        class_id: i64,
        region: Region,
    ) -> Result<PlayableClassResponse, ServerFnError> {
        let playable_class = client.call_async(PlayableClassRequest { class_id }).await?;
        Ok(playable_class)
    }

    async fn inner(
        storage: Storage,
        client: impl BattleNetClientAsync + Clone + Send + Sync,
        character_name: String,
        region: Region,
    ) -> Result<Vec<RealmClassEntry>, ServerFnError> {
        let realms = storage
            .try_fetch(&(region,), Duration::from_secs(24 * 60 * 60), move || {
                realms(region)
            })
            .await?;
        let realms_and_validity = realms.into_iter().map({
            move |r| {
                let character_name = character_name.clone();
                let client = client.clone();
                let storage = storage.clone();
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

                    let summary =
                        character_summary(character_name.clone(), realm_slug.clone(), region)
                            .await
                            .ok()?;

                    let class_id = summary.character_class.id;
                    let class = storage
                        .clone()
                        .try_fetch(
                            &(class_id, region),
                            Duration::from_secs(24 * 60 * 60),
                            move || {
                                playable_class(storage.clone(), client.clone(), class_id, region)
                            },
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
            }
        });

        let realms = join_all(realms_and_validity).await;
        let valid_realms = realms.into_iter().flatten().collect();

        Ok(valid_realms)
    }

    let storage = get_storage().await?;
    let client = get_bnet_client(region, ClientType::AllowFallback).await?;
    storage
        .clone()
        .try_fetch(
            &(character_name.clone(), region),
            Duration::from_secs(24 * 60 * 60),
            move || inner(storage.clone(), client, character_name, region),
        )
        .await
}
