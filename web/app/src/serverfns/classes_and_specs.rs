use std::collections::HashMap;
use std::time::Duration;

use futures_util::future::try_join_all;
use leptos::prelude::*;
use leptos::server_fn::codec::{Cbor, GetUrl, Json};
use tracing::instrument;

use auto_battle_net::BattleNetClientAsync;
use auto_battle_net::game_data::journal::journal_encounter::{
    JournalEncounterRequest, JournalEncounterResponse,
};
use auto_battle_net::game_data::playable_class::playable_class::{
    PlayableClassRequest, PlayableClassResponse,
};
use auto_battle_net::game_data::playable_class::playable_classes_index::{
    PlayableClassesIndexRequest, PlayableClassesIndexResponse,
};
use auto_battle_net::game_data::playable_specialization::playable_specialization::{
    PlayableSpecializationRequest, PlayableSpecializationResponse,
};
use auto_battle_net::game_data::playable_specialization::playable_specializations_index::{
    PlayableSpecializationsIndexRequest, PlayableSpecializationsIndexResponse,
};
use fight_domain::{Attack, Lookup};
use i18n::LocalizedString;

#[instrument]
#[server(prefix = "/bnet", input = GetUrl, output = Json)]
pub async fn classes_and_specs(
) -> Result<Vec<(LocalizedString, Vec<LocalizedString>)>, ServerFnError> {
    use crate::serverfns::util::{get_bnet_client_regionless, get_storage, ClientType};
    let client = get_bnet_client_regionless(ClientType::AllowFallback).await?;

    async fn inner(client: impl BattleNetClientAsync + Clone + Send + Sync) -> Result<Vec<(LocalizedString, Vec<LocalizedString>)>, ServerFnError> {
        let class_index = client.call_async(PlayableClassesIndexRequest {});
        let spec_index = client.call_async(PlayableSpecializationsIndexRequest {});

        let promises = class_index.await?.classes.into_iter().map(|c| {
            let client = client.clone();
            async move {
                let class_info = client.call_async(PlayableClassRequest { class_id: c.id }).await?;
                let specs = class_info
                    .specializations
                    .into_iter()
                    .map(|s| s.name)
                    .collect::<Vec<_>>();
                Ok::<_, ServerFnError>((c.name, specs))
            }
        });

        Ok(try_join_all(promises).await?.into_iter().collect())
    }

    let storage = get_storage().await?;
    storage.try_fetch(&("classes_and_specs".to_string(),), Duration::from_secs(60 * 60), move || async move { inner(client).await }).await
}
