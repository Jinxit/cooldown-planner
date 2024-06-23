use axum::extract::Path;
use axum::response::IntoResponse;
use url::Url;

use auto_battle_net::{BattleNetClientAsync, ReqwestBattleNetClient};
//use cached::proc_macro::cached;
use auto_battle_net::game_data::spell::spell_media::SpellMediaRequest;
use battle_net_auth::OAuthToken;
use i18n::Region;

#[axum::debug_handler]
pub async fn spell_icon(Path(spell_id): Path<i64>, access_token: OAuthToken) -> impl IntoResponse {
    async fn inner(access_token: OAuthToken, spell_id: i64) -> Option<Url> {
        let request = SpellMediaRequest { spell_id };
        let token = access_token.expose_secret();
        let icon_url = ReqwestBattleNetClient {
            region: Region::Europe,
            access_token: token.clone(),
        }
        .call_async(request)
        .await
        .ok()?
        .assets
        .iter()
        .find(|asset| asset.key == "icon")
        .cloned()
        .map(|asset| asset.value)
        .and_then(|url| Url::parse(&url).ok())?;

        Some(icon_url)
    }

    let icon_url = inner(access_token.clone(), spell_id)
        .await
        .unwrap_or_else(|| {
            Url::parse("https://render.worldofwarcraft.com/eu/icons/56/inv_misc_questionmark.jpg")
                .expect("static url should always parse successfully")
        });

    crate::reverse_proxy::reverse_proxy(icon_url, Some(access_token)).await
}
