use std::time::Duration;

use crate::serverfns::{encounter_info, instance_info};
use auto_battle_net::game_data::journal::{
    journal_encounter::JournalEncounterResponse, journal_instance::JournalInstanceResponse,
};
use futures_util::try_join;
use leptos::*;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Clone, Serialize, Deserialize)]
pub struct AberrusInfo {
    pub instance: JournalInstanceResponse,
    pub kazzara: JournalEncounterResponse,
    pub amalgamation: JournalEncounterResponse,
    pub experiments: JournalEncounterResponse,
    pub assault: JournalEncounterResponse,
    pub rashok: JournalEncounterResponse,
    pub zskarn: JournalEncounterResponse,
    pub magmorax: JournalEncounterResponse,
    pub neltharion: JournalEncounterResponse,
    pub sarkareth: JournalEncounterResponse,
}

#[instrument]
#[server(Aberrus, "/bnet", "GetCbor")]
pub async fn aberrus() -> Result<AberrusInfo, ServerFnError> {
    use crate::serverfns::util::try_fetch_cached;

    async fn inner() -> Result<AberrusInfo, ServerFnError> {
        let instance = instance_info(1208);
        let kazzara = encounter_info(2522);
        let amalgamation = encounter_info(2529);
        let experiments = encounter_info(2530);
        let assault = encounter_info(2524);
        let rashok = encounter_info(2525);
        let zskarn = encounter_info(2532);
        let magmorax = encounter_info(2527);
        let neltharion = encounter_info(2523);
        let sarkareth = encounter_info(2520);

        let (
            instance,
            kazzara,
            amalgamation,
            experiments,
            assault,
            rashok,
            zskarn,
            magmorax,
            neltharion,
            sarkareth,
        ) = try_join!(
            instance,
            kazzara,
            amalgamation,
            experiments,
            assault,
            rashok,
            zskarn,
            magmorax,
            neltharion,
            sarkareth
        )?;

        Ok(AberrusInfo {
            instance,
            kazzara,
            amalgamation,
            experiments,
            assault,
            rashok,
            zskarn,
            magmorax,
            neltharion,
            sarkareth,
        })
    }

    try_fetch_cached(&("Aberrus"), Duration::from_secs(24 * 60 * 60), inner).await
}
