use crate::components::fights::Difficulty;
use auto_battle_net::game_data::journal::journal_encounter::{
    JournalEncounterRequest, JournalEncounterResponse,
};
use auto_battle_net::game_data::journal::journal_instance::JournalInstanceResponse;
use auto_battle_net::Region;
use auto_battle_net::{BattleNetClientAsync, LocalizedString};
//use cached::proc_macro::cached;
use fight_domain::{Attack, Lookup};
use leptos::{server, ServerFnError};
use leptos::{IntoView, Signal, View};
use tracing::instrument;

#[instrument]
#[server(EncounterInfo, "/bnet", "GetCbor")]
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
