//use cached::proc_macro::cached;
use convert_case::{Case, Casing};
use leptos::prelude::*;
use leptos::server_fn::codec::{Cbor, GetUrl, Json};
use server_fn::error::NoCustomError;
use tracing::{error, instrument, warn};
use url::Url;

use auto_battle_net::BattleNetClientAsync;
use auto_battle_net::profile::character_media::character_media_summary::CharacterMediaSummaryRequest;
use i18n::Region;

#[instrument]
#[server(prefix = "/bnet", input = GetUrl, output = Cbor)]
pub async fn character_main_image(
    character_name: String,
    realm_slug: String,
    region: Region,
) -> Result<Url, ServerFnError> {
    use crate::serverfns::util::{get_bnet_client, ClientType};

    async fn inner(
        character_name: String,
        realm_slug: String,
        region: Region,
    ) -> Result<Url, ServerFnError> {
        let client = get_bnet_client(region, ClientType::AllowFallback).await?;
        let summary = client
            .call_async(CharacterMediaSummaryRequest {
                realm_slug,
                character_name: character_name.to_case(Case::Kebab),
            })
            .await?;
        let main = summary
            .assets
            .into_iter()
            .find(|asset| asset.key == "main-raw")
            .ok_or_else(|| {
                ServerFnError::ServerError::<NoCustomError>(format!(
                    "main image not found for character {character_name}"
                ))
            })?
            .value;

        let url = Url::parse(&main)?;

        Ok(url)
    }
    inner(character_name, realm_slug, region).await
}
