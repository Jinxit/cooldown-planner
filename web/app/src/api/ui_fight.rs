use crate::components::fights::Difficulty;
use crate::serverfns::encounter_info;
use auto_battle_net::game_data::journal::journal_encounter::{
    JournalEncounterRequest, JournalEncounterResponse,
};
use auto_battle_net::game_data::journal::journal_instance::JournalInstanceResponse;
use auto_battle_net::{BattleNetClientAsync, LocalizedString};
use fight_domain::{Attack, Lookup};
use leptos::{IntoView, Signal, View};
use serde::{Deserialize, Serialize};

pub trait FightProps {
    fn new() -> Self;
}

impl FightProps for () {
    fn new() -> Self {}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UiFight {
    pub instance_id: i64,
    pub instance_name: LocalizedString,
    pub encounter_id: i64,
    pub encounter_name: LocalizedString,
    pub encounter_description: LocalizedString,
    pub difficulty: Difficulty,
    pub parameters: View,
    pub attacks: Signal<Lookup<Attack>>,
    pub image_path: &'static str,
    pub image_offset: i32,
}

impl UiFight {
    #[allow(clippy::too_many_arguments)]
    pub fn new<P: FightProps + Copy, IV: IntoView>(
        instance_info: &JournalInstanceResponse,
        encounter_info: &JournalEncounterResponse,
        difficulty: Difficulty,
        parameters: fn(P) -> IV,
        attacks: fn(P) -> Signal<Lookup<Attack>>,
        image_path: &'static str,
        image_offset: i32,
    ) -> UiFight {
        let props = FightProps::new();
        UiFight {
            instance_id: instance_info.id,
            instance_name: instance_info.name.clone(),
            encounter_id: encounter_info.id,
            encounter_name: encounter_info.name.clone(),
            encounter_description: encounter_info.description.clone(),
            difficulty,
            parameters: parameters(props).into_view(),
            attacks: attacks(props),
            image_path,
            image_offset,
        }
    }
}
