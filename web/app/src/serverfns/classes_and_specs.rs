use std::collections::HashMap;

use crate::components::fights::Difficulty;
use auto_battle_net::game_data::journal::journal_encounter::{
    JournalEncounterRequest, JournalEncounterResponse,
};
use auto_battle_net::game_data::journal::journal_instance::{
    JournalInstanceRequest, JournalInstanceResponse,
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
use auto_battle_net::Region;
use auto_battle_net::{BattleNetClientAsync, LocalizedString};
//use cached::proc_macro::cached;
use fight_domain::{Attack, Lookup};
use futures_util::future::try_join_all;
use leptos::prelude::*;
use tracing::instrument;
use leptos::server_fn::codec::{GetUrl, Json, Cbor};

#[instrument]
#[server(prefix = "/bnet", input = GetUrl, output = Json)]
pub async fn classes_and_specs(
) -> Result<Vec<(LocalizedString, Vec<LocalizedString>)>, ServerFnError> {
    use crate::serverfns::util::get_bnet_client_regionless;

    async fn inner() -> Result<Vec<(LocalizedString, Vec<LocalizedString>)>, ServerFnError> {
        let client = get_bnet_client_regionless().await?;
        let class_index = client.call_async(PlayableClassesIndexRequest {});
        let spec_index = client.call_async(PlayableSpecializationsIndexRequest {});

        let promises = class_index.await?.classes.into_iter().map(|c| {
            let client = client.clone();
            async move {
                let class_info = client
                    .call_async(PlayableClassRequest { class_id: c.id })
                    .await?;
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

    inner().await
}
