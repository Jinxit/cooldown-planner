use std::time::Duration;

use convert_case::{Case, Casing};
use leptos::prelude::*;
use leptos::server_fn::codec::{GetUrl, Json};
use tracing::instrument;

use auto_battle_net::BattleNetClientAsync;
use auto_battle_net::profile::character_profile::character_profile_summary::{
    CharacterProfileSummaryRequest, CharacterProfileSummaryResponse,
};
use i18n::Region;

#[instrument]
#[server(prefix = "/bnet", input = GetUrl, output = Json)]
pub async fn character_summary(
    character_name: String,
    realm_slug: String,
    region: Region,
) -> Result<CharacterProfileSummaryResponse, ServerFnError> {
    use super::util::{get_bnet_client, get_storage, ClientType};
    use auto_battle_net::ReqwestBattleNetClient;

    async fn inner(
        character_name: String,
        realm_slug: String,
        region: Region,
    ) -> Result<CharacterProfileSummaryResponse, ServerFnError> {
        let client = get_bnet_client(region, ClientType::AllowFallback).await?;
        let summary = client
            .call_async(CharacterProfileSummaryRequest {
                realm_slug,
                character_name: character_name.to_case(Case::Kebab),
            })
            .await?;
        Ok(summary)
    }
    let storage = get_storage().await?;
    storage.try_fetch(
        &(character_name.clone(), realm_slug.clone(), region),
        Duration::from_secs(24 * 60 * 60),
        move || inner(character_name, realm_slug, region),
    )
    .await
}
