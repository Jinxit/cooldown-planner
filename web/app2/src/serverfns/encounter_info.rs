use leptos::*;
use leptos::server_fn::codec::{Cbor, GetUrl, Json};
use tracing::instrument;

use auto_battle_net::BattleNetClientAsync;
use auto_battle_net::game_data::journal::journal_encounter::{
    JournalEncounterRequest, JournalEncounterResponse,
};
use auto_battle_net::game_data::journal::journal_instance::JournalInstanceResponse;
//use cached::proc_macro::cached;
use fight_domain::{Attack, Lookup};

use crate::components::fights::Difficulty;

#[instrument]
#[server(prefix = "/bnet", input = GetUrl, output = Json)]
pub async fn encounter_info(encounter_id: i64) -> Result<JournalEncounterResponse, ServerFnError> {
    use crate::serverfns::util::get_bnet_client_regionless;

    async fn inner(encounter_id: i64) -> Result<JournalEncounterResponse, ServerFnError> {
        let client = get_bnet_client_regionless().await?;
        let journal_encounter = client
            .call_async(JournalEncounterRequest {
                journal_encounter_id: encounter_id,
            })
            .await?;
        Ok(journal_encounter)
    }
    inner(encounter_id).await
}
