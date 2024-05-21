use std::time::Duration;

use crate::api::fuzzy_search::{Searchable, Term};
use crate::components::fights::Difficulty;
use crate::context::{PlannerRealm, PlayerClass};
use auto_battle_net::game_data::playable_class::playable_class::{
    PlayableClassRequest, PlayableClassResponse,
};
use auto_battle_net::profile::guild::guild_roster::{GuildRosterRequest, GuildRosterResponse};
use auto_battle_net::Region;
use auto_battle_net::{BattleNetClientAsync, LocalizedString};
//use cached::proc_macro::cached;
use fight_domain::{Attack, Lookup};
use futures_util::future::try_join_all;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::error::NoCustomError;
use tracing::instrument;
use leptos::server_fn::codec::{GetUrl, Json, Cbor};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GuildRosterEntry {
    pub name: String,
    pub class: PlayerClass,
}

#[instrument]
#[server(prefix = "/bnet", input = GetUrl, output = Json)]
pub async fn guild_roster(
    realm_slug: String,
    guild_name_slug: String,
    region: Region,
) -> Result<Vec<GuildRosterEntry>, ServerFnError> {
    use crate::serverfns::util::{get_bnet_client, get_session, try_fetch_cached};

    async fn playable_class(
        class_id: i64,
        region: Region,
    ) -> Result<PlayableClassResponse, ServerFnError> {
        let client = get_bnet_client(region).await?;
        let playable_class = client.call_async(PlayableClassRequest { class_id }).await?;
        Ok(playable_class)
    }

    async fn inner(
        realm_slug: String,
        guild_name_slug: String,
        region: Region,
    ) -> Result<Vec<GuildRosterEntry>, ServerFnError> {
        let client = get_bnet_client(region).await?;
        let guild_roster = client
            .call_async(GuildRosterRequest {
                realm_slug: realm_slug.clone(),
                name_slug: guild_name_slug,
            })
            .await?;
        let futures = try_join_all(guild_roster.members.into_iter().filter_map({
            move |m| {
                if m.character.level == 70 {
                    Some(async move {
                        let class_id = m.character.playable_class.id;
                        let class = try_fetch_cached(
                            &(class_id, region),
                            Duration::from_secs(24 * 60 * 60),
                            move || playable_class(class_id, region),
                        )
                        .await?
                        .name;
                        Ok::<_, ServerFnError>(GuildRosterEntry {
                            name: m.character.name,
                            class: PlayerClass::Known(class),
                        })
                    })
                } else {
                    None
                }
            }
        }))
        .await?;

        Ok(futures)
    }

    let user_id = get_session()
        .await
        .ok_or_else(|| ServerFnError::MissingArg::<NoCustomError>("session".to_string()))?;

    try_fetch_cached(
        &(realm_slug.clone(), guild_name_slug.clone(), region),
        Duration::from_secs(24 * 60 * 60),
        move || inner(realm_slug, guild_name_slug, region),
    )
    .await
}
