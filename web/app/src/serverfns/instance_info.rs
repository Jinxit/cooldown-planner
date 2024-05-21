use crate::components::fights::Difficulty;
use auto_battle_net::game_data::journal::journal_encounter::{
    JournalEncounterRequest, JournalEncounterResponse,
};
use auto_battle_net::game_data::journal::journal_instance::{
    JournalInstanceRequest, JournalInstanceResponse,
};
use auto_battle_net::Region;
use auto_battle_net::{BattleNetClientAsync, LocalizedString};
//use cached::proc_macro::cached;
use fight_domain::{Attack, Lookup};
use leptos::prelude::*;
use tracing::instrument;
use leptos::server_fn::codec::{GetUrl, Json, Cbor};

#[instrument]
#[server(prefix = "/bnet", input = GetUrl, output = Json)]
pub async fn instance_info(instance_id: i64) -> Result<JournalInstanceResponse, ServerFnError> {
    use crate::serverfns::util::get_bnet_client_regionless;

    async fn inner(instance_id: i64) -> Result<JournalInstanceResponse, ServerFnError> {
        let client = get_bnet_client_regionless().await?;
        let journal_instance = client
            .call_async(JournalInstanceRequest {
                journal_instance_id: instance_id,
            })
            .await?;
        Ok(journal_instance)
    }

    inner(instance_id).await
}
