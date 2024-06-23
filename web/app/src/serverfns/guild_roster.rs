use std::time::Duration;

use futures_util::{stream, StreamExt, TryStreamExt};
use futures_util::future::try_join_all;
use futures_util::stream::FuturesUnordered;
use leptos::prelude::*;
use leptos::server_fn::codec::{Cbor, GetUrl, Json};
use serde::{Deserialize, Serialize};
use server_fn::error::NoCustomError;
use tracing::instrument;

use auto_battle_net::BattleNetClientAsync;
use auto_battle_net::game_data::playable_class::playable_class::{
    PlayableClassRequest, PlayableClassResponse,
};
use auto_battle_net::profile::guild::guild_roster::{GuildRosterRequest, GuildRosterResponse};
//use cached::proc_macro::cached;
use fight_domain::{Attack, Lookup};
use i18n::{LocalizedString, Region};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum PlayerClass {
    Known(LocalizedString),
    Unknown,
    Missing,
}

impl PlayerClass {
    pub fn opt(&self) -> Option<&LocalizedString> {
        match self {
            PlayerClass::Known(name) => Some(name),
            PlayerClass::Unknown => None,
            PlayerClass::Missing => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GuildRosterEntry {
    pub name: String,
    pub class: LocalizedString,
}

#[server(prefix = "/bnet", input = GetUrl, output = Json)]
pub async fn guild_roster(
    realm_slug: String,
    guild_name_slug: String,
    region: Region,
) -> Result<Vec<GuildRosterEntry>, ServerFnError> {
    use crate::serverfns::util::{get_bnet_client, get_storage, ClientType};
    use storage::Storage;

    let client = get_bnet_client(region, ClientType::AllowFallback).await?;

    #[instrument(skip(client))]
    async fn playable_class(
        client: impl BattleNetClientAsync + Clone + Send + Sync,
        class_id: i64,
        region: Region,
    ) -> Result<PlayableClassResponse, ServerFnError> {
        let playable_class = client.call_async(PlayableClassRequest { class_id }).await?;
        Ok(playable_class)
    }

    #[instrument(skip(client))]
    async fn inner(
        client: impl BattleNetClientAsync + Clone + Send + Sync,
        storage: Storage,
        realm_slug: String,
        guild_name_slug: String,
        region: Region,
    ) -> Result<Vec<GuildRosterEntry>, ServerFnError> {
        let guild_roster = client
            .call_async(GuildRosterRequest {
                realm_slug: realm_slug.clone(),
                name_slug: guild_name_slug,
            })
            .await?;
        let entries: Vec<_> = guild_roster.members.into_iter().filter_map({
            let storage = storage.clone();
            move |m| {
                if m.character.level == 70 {
                    let client = client.clone();
                    let storage = storage.clone();
                    Some(async move {
                        let class_id = m.character.playable_class.id;
                        let class = storage.try_fetch(
                            &(class_id,),
                            Duration::from_secs(24 * 60 * 60),
                            move || playable_class(client.clone(), class_id, region),
                        )
                        .await?
                        .name;
                        Ok::<_, ServerFnError>(GuildRosterEntry {
                            name: m.character.name,
                            class,
                        })
                    })
                } else {
                    None
                }
            }
        })
        .collect::<FuturesUnordered<_>>()
        .try_collect()
        .await?;

        Ok(entries)
    }

    let storage = get_storage().await?;
    storage.clone().try_fetch(
        &(realm_slug.clone(), guild_name_slug.clone(), region),
        Duration::from_secs(24 * 60 * 60),
        move || inner(client, storage.clone(), realm_slug, guild_name_slug, region),
    )
    .await
}
