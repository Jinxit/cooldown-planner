//use cached::proc_macro::cached;
use convert_case::{Case, Casing};
use leptos::*;
use leptos::server_fn::codec::{Cbor, GetUrl, Json};
use server_fn::error::NoCustomError;
use tracing::{error, instrument, warn};
use url::Url;

use auto_battle_net::BattleNetClientAsync;
use auto_battle_net::profile::character_media::character_media_summary::CharacterMediaSummaryRequest;
use i18n::Region;

use crate::api::ui_character::UiCharacter;
use crate::context::{PlannerContext, PlannerRealm};
use crate::misc::localized_string_with_context::LocalizedStringWithContext;

#[instrument]
#[server(prefix = "/bnet", input = GetUrl, output = Cbor)]
pub async fn character_avatar(
    character_name: String,
    realm_slug: String,
    region: Region,
) -> Result<Url, ServerFnError> {
    use crate::serverfns::util::get_bnet_client;

    async fn inner(
        character_name: String,
        realm_slug: String,
        region: Region,
    ) -> Result<Url, ServerFnError> {
        let client = get_bnet_client(region).await?;
        let summary = client
            .call_async(CharacterMediaSummaryRequest {
                realm_slug,
                character_name: character_name.to_case(Case::Kebab),
            })
            .await?;

        let avatar = summary
            .assets
            .into_iter()
            .find(|asset| asset.key == "avatar")
            .ok_or_else(|| {
                ServerFnError::ServerError::<NoCustomError>(format!(
                    "avatar not found for character {character_name}"
                ))
            })?
            .value;

        let url = Url::parse(&avatar)?;

        Ok(url)
    }

    inner(character_name, realm_slug, region).await
}
