use crate::api::ui_character::UiCharacter;
use crate::context::{PlannerContext, PlannerRealm};
use crate::misc::localized_string_with_context::LocalizedStringWithContext;
use crate::reactive::map::Map;
use auto_battle_net::profile::character_media::character_media_summary::CharacterMediaSummaryRequest;
use auto_battle_net::{BattleNetClientAsync, Locale, Region};
//use cached::proc_macro::cached;
use convert_case::{Case, Casing};
use leptos::*;
use tracing::{error, instrument, warn};
use url::Url;

#[instrument]
#[server(CharacterAvatar, "/bnet", "GetCbor")]
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
                ServerFnError::ServerError(format!(
                    "avatar not found for character {character_name}"
                ))
            })?
            .value;

        let url = Url::parse(&avatar)?;

        Ok(url)
    }

    inner(character_name, realm_slug, region).await
}
