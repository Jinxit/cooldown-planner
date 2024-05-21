use crate::components::fights::Difficulty;
use auto_battle_net::game_data::journal::journal_encounter::{
    JournalEncounterRequest, JournalEncounterResponse,
};
use auto_battle_net::game_data::journal::journal_instance::JournalInstanceResponse;
use auto_battle_net::game_data::realm::realms_index::{
    Realms, RealmsIndexRequest, RealmsIndexResponse,
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
pub async fn region_realms(region: Region) -> Result<Vec<Realms>, ServerFnError> {
    use super::util::get_bnet_client;

    async fn inner(region: Region) -> Result<Vec<Realms>, ServerFnError> {
        let client = get_bnet_client(region).await?;
        let realms = client.call_async(RealmsIndexRequest {}).await?;
        Ok(realms.realms)
    }
    inner(region).await
}
