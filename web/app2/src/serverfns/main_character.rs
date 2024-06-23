use std::time::Duration;

//use cached::proc_macro::cached;
use convert_case::{Case, Casing};
use futures_util::future::{join_all, try_join_all};
use futures_util::StreamExt;
use itertools::Itertools;
use leptos::*;
use leptos::server_fn::codec::{Cbor, GetUrl, Json, PostUrl};
use num_traits::Zero;
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};
use server_fn::error::NoCustomError;
use strum::IntoEnumIterator;
use tracing::{error, instrument};

use auto_battle_net::BattleNetClientAsync;
use auto_battle_net::game_data::realm::realm::RealmRequest;
use auto_battle_net::profile::account_profile::account_profile_summary::AccountProfileSummaryRequest;
use auto_battle_net::profile::character_encounters::character_raids::CharacterRaidsRequest;
use auto_battle_net::profile::character_mythic_keystone_profile::character_mythic_keystone_profile_index::CharacterMythicKeystoneProfileIndexRequest;
use auto_battle_net::profile::character_profile::character_profile_summary::CharacterProfileSummaryRequest;
use i18n::{Locale, Region};

use crate::context::{PlannerRealm, PlannerUser};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CharacterDetails {
    pub level: i64,
    pub raid_progress: Vec<NotNan<f64>>,
    pub mythic_plus_period: i64,
    pub mythic_plus_rating: NotNan<f64>,
    pub region: Region,
    pub locale: Locale,
    pub name: String,
    pub realm: PlannerRealm,
    pub guild: Option<String>,
    pub id: i64,
}

#[instrument]
#[server(CurrentMainCharacter, "/bnet", input = GetUrl, output = Json)]
pub async fn current_main_character() -> Result<CharacterDetails, ServerFnError> {
    use crate::serverfns::util::{get_bnet_client, get_session, get_storage};

    let user = get_session()
        .await
        .ok_or_else(|| ServerFnError::MissingArg::<NoCustomError>("session".to_string()))?
        .data()
        .user
        .clone()
        .ok_or_else(|| ServerFnError::MissingArg::<NoCustomError>("user".to_string()))?;

    async fn inner(user_id: u64) -> Result<CharacterDetails, ServerFnError> {
        let characters = all_characters_sorted(user_id).await?;
        characters
            .first()
            .cloned()
            .ok_or_else(|| ServerFnError::Request::<NoCustomError>("No characters".to_string()))
    }

    let storage = get_storage().await;
    storage
        .try_fetch(
            &format!("MainCharacter/{}", user.id),
            Duration::from_secs(24 * 60 * 60),
            move || async move { inner(user.id).await },
        )
        .await
}

#[instrument]
#[server(prefix = "/bnet", input = PostUrl)]
pub async fn set_main_character(
    region: Region,
    name: String,
    realm: PlannerRealm,
) -> Result<(), ServerFnError> {
    use crate::serverfns::util::{get_bnet_client, get_session, get_storage};

    let user = get_session()
        .await
        .ok_or_else(|| ServerFnError::MissingArg::<NoCustomError>("session".to_string()))?
        .data()
        .user
        .clone()
        .ok_or_else(|| ServerFnError::MissingArg::<NoCustomError>("user".to_string()))?;

    let storage = get_storage().await;

    let details = character_details(region, name, realm).await?;
    storage
        .put(
            &format!("MainCharacter/{}", user.id),
            Duration::from_secs(365 * 24 * 60 * 60),
            &details,
        )
        .await;

    Ok(())
}

#[instrument]
#[cfg(feature = "ssr")]
async fn all_characters_sorted(user_id: u64) -> Result<Vec<CharacterDetails>, ServerFnError> {
    use crate::serverfns::util::get_bnet_client;

    let futures = Region::iter().map(|r| async move {
        let client = get_bnet_client(r).await?;
        let account_profile = client.call_async(AccountProfileSummaryRequest {}).await?;

        Result::<_, ServerFnError>::Ok(
            join_all(
                account_profile
                    .wow_accounts
                    .iter()
                    .flat_map(|acc| acc.characters.iter())
                    .map(|c| async move {
                        let realm = PlannerRealm {
                            name: c.realm.name.clone(),
                            slug: c.realm.slug.clone(),
                        };

                        character_details(r, c.name.clone(), realm.clone()).await
                    }),
            )
            .await
            .into_iter()
            .filter_map(|p| p.ok()),
        )
    });

    let mut characters: Vec<CharacterDetails> = join_all(futures)
        .await
        .into_iter()
        .filter_map(|p| p.ok())
        .flat_map(|it| it.into_iter())
        .collect();
    characters.sort_by(|a, b| b.cmp(a));
    Ok(characters)
}

#[instrument]
#[cfg(feature = "ssr")]
async fn character_details(
    region: Region,
    name: String,
    realm: PlannerRealm,
) -> Result<CharacterDetails, ServerFnError> {
    use crate::serverfns::util::get_bnet_client;

    let client = get_bnet_client(region).await?;
    let character_slug = name.to_case(Case::Kebab);

    let current_raid_progress = async {
        let raids = client
            .call_async(CharacterRaidsRequest {
                character_name: character_slug.clone(),
                realm_slug: realm.slug.clone(),
            })
            .await?;

        Ok::<_, ServerFnError>(
            raids
                .expansions
                .iter()
                .find(|e| e.expansion.id == 503) // dragonflight
                .and_then(|e| e.instances.iter().find(|i| i.instance.id == 1208)) // aberrus
                .map(|i| {
                    i.modes
                        .iter()
                        .filter(|m| m.difficulty.r#type != "LFR")
                        .map(|m| {
                            NotNan::new(m.progress.completed_count as f64).unwrap()
                                / NotNan::new(m.progress.total_count as f64).unwrap()
                        })
                        .rev()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
        )
    };

    let mythic_plus = client.call_async(CharacterMythicKeystoneProfileIndexRequest {
        character_name: character_slug.clone(),
        realm_slug: realm.slug.clone(),
    });

    let character_profile = client.call_async(CharacterProfileSummaryRequest {
        character_name: character_slug.clone(),
        realm_slug: realm.slug.clone(),
    });

    let realm_details = client.call_async(RealmRequest {
        realm_slug: realm.slug.clone(),
    });

    let character_profile = character_profile.await?;
    let mythic_plus = mythic_plus.await?;
    let realm_details = realm_details.await?;
    let current_raid_progress = current_raid_progress.await?;

    let details = CharacterDetails {
        level: character_profile.level,
        raid_progress: current_raid_progress,
        mythic_plus_period: mythic_plus.current_period.period.id,
        mythic_plus_rating: mythic_plus
            .current_mythic_rating
            .map(|r| r.rating)
            .unwrap_or(NotNan::zero()),
        region,
        locale: realm_details.locale,
        name,
        realm,
        guild: character_profile.guild.map(|g| g.name),
        id: character_profile.id,
    };

    Ok(details)
}
