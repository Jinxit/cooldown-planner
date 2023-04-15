use crate::api::ui_fight::{FightProps, UiFight};
use crate::misc::flatten_ok::FlattenOk;
use crate::reactive::resource_ext::ResourceAndThenExt;
use crate::serverfns::{aberrus, encounter_info, instance_info};
use auto_battle_net::game_data::journal::journal_encounter::JournalEncounterResponse;
use auto_battle_net::game_data::journal::journal_instance::{
    JournalInstanceRequest, JournalInstanceResponse,
};
use auto_battle_net::BattleNetClientAsync;
use fight_domain::Lookup;
use leptos::*;

mod amalgamation;
mod assault;
mod experiments;
mod kazzara;
mod rashok;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Difficulty {
    Heroic,
    Mythic,
}

pub fn mythic_aberrus() -> Signal<Vec<UiFight>> {
    let info = create_resource(|| (), move |_| aberrus());

    Signal::derive(move || {
        info.and_then(|info| {
            vec![
                kazzara::mythic(&info.instance, &info.kazzara),
                amalgamation::mythic(&info.instance, &info.amalgamation),
                experiments::mythic(&info.instance, &info.experiments),
                assault::mythic(&info.instance, &info.assault),
                rashok::mythic(&info.instance, &info.rashok),
                zskarn_mythic(&info.instance, &info.zskarn),
                magmorax_mythic(&info.instance, &info.magmorax),
                neltharion_mythic(&info.instance, &info.neltharion),
                sarkareth_mythic(&info.instance, &info.sarkareth),
            ]
        })
        .flatten_ok()
        .unwrap_or_default()
    })
}

pub fn sarkareth_mythic(
    instance_info: &JournalInstanceResponse,
    encounter_info: &JournalEncounterResponse,
) -> UiFight {
    UiFight::new(
        instance_info,
        encounter_info,
        Difficulty::Mythic,
        |props| view! {  () },
        |_: ()| Signal::derive(Lookup::default),
        "boss/sarkareth.png",
        40,
    )
}

pub fn neltharion_mythic(
    instance_info: &JournalInstanceResponse,
    encounter_info: &JournalEncounterResponse,
) -> UiFight {
    UiFight::new(
        instance_info,
        encounter_info,
        Difficulty::Mythic,
        |props| view! {  () },
        |_: ()| Signal::derive(Lookup::default),
        "boss/neltharion.png",
        13,
    )
}

pub fn magmorax_mythic(
    instance_info: &JournalInstanceResponse,
    encounter_info: &JournalEncounterResponse,
) -> UiFight {
    UiFight::new(
        instance_info,
        encounter_info,
        Difficulty::Mythic,
        |props| view! {  () },
        |_: ()| Signal::derive(Lookup::default),
        "boss/magmorax.png",
        26,
    )
}

pub fn zskarn_mythic(
    instance_info: &JournalInstanceResponse,
    encounter_info: &JournalEncounterResponse,
) -> UiFight {
    UiFight::new(
        instance_info,
        encounter_info,
        Difficulty::Mythic,
        |props| view! {  () },
        |_: ()| Signal::derive(Lookup::default),
        "boss/zskarn.png",
        22,
    )
}
