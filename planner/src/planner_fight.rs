use auto_battle_net::game_data::journal::journal_encounter::JournalEncounterResponse;
use auto_battle_net::game_data::journal::journal_instance::JournalInstanceResponse;
use fight_domain::{Attack, Lookup};
use i18n::LocalizedString;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Difficulty {
    Heroic,
    Mythic,
}

pub trait PlannerFight: Send + Sync {
    fn data(&self) -> &Option<PlannerFightData>;
    fn attacks(&self) -> Lookup<Attack>;
}

#[derive(Clone)]
pub struct PlannerFightData {
    pub instance_id: i64,
    pub instance_name: LocalizedString,
    pub encounter_id: i64,
    pub encounter_name: LocalizedString,
    pub encounter_description: LocalizedString,
    pub difficulty: Difficulty,
    pub image_path: &'static str,
    pub image_offset: i32,
}

impl PlannerFightData {
    pub fn new(
        instance_info: &JournalInstanceResponse,
        encounter_info: &JournalEncounterResponse,
        difficulty: Difficulty,
        image_path: &'static str,
        image_offset: i32,
    ) -> Self {
        Self {
            instance_id: instance_info.id,
            instance_name: instance_info.name.clone(),
            encounter_id: encounter_info.id,
            encounter_name: encounter_info.name.clone(),
            encounter_description: encounter_info.description.clone(),
            difficulty,
            image_path,
            image_offset,
        }
    }
}
