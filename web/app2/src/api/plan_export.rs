use std::fmt::{Display, Formatter};

use itertools::Itertools;

use fight_domain::{Identifier, TimeStep};
use optimizer::Plan;

pub trait PlanExt {
    fn export(&self) -> Vec<String>;
}

impl PlanExt for Plan {
    fn export(&self) -> Vec<String> {
        self.assignments
            .iter()
            .sorted_by_key(|assignment| self.attacks.get(&assignment.attack).unwrap().timer.static_timer())
            .chunk_by(|assignment| assignment.attack)
            .into_iter()
            .map(|(attack_uuid, assignments)| {
                let characters = assignments
                    .sorted_by_key(|assignment| self.characters.get(&assignment.character).unwrap().name.clone())
                    .chunk_by(|assignment| assignment.character)
                    .into_iter()
                    .map(|(character_uuid, assignments)| {
                        let character_name = self.characters.get(&character_uuid).unwrap().name.clone();
                        [character_name].into_iter().chain(assignments.into_iter().map(|assignment| {
                            let identifier = self.characters.get(&character_uuid).unwrap().spells.get(&assignment.spell).unwrap().identifier.clone();
                            identifier.in_game_note().to_string()
                        })).join(" ")
                    })
                    .join("  ");
                let attack = self.attacks.get(&attack_uuid).unwrap();
                let dynamic_timer = attack.timer.dynamic_timer.unwrap_or(TimeStep::zero());
                let spell_trigger = match &attack.timer.dynamic_trigger_cleu_event {
                    Some(cleu_event) => {
                        let event_type = &cleu_event.r#type;
                        let event_id = cleu_event.event;
                        let counter = cleu_event.counter;
                        format!(",{event_type}:{event_id}:{counter}")
                    },
                    None => "".to_string(),
                };
                let static_timer = attack.timer.static_timer();
                let attack_name = &attack.name;
                format!("{{time:{dynamic_timer}{spell_trigger}}}{static_timer} - {attack_name} - {characters}")
            })
            .collect::<Vec<String>>()
    }
}

pub trait AsInGameNote {
    fn in_game_note(&self) -> InGameNoteFormat;
}

pub struct InGameNoteFormat(String);

impl Display for InGameNoteFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsInGameNote for Identifier {
    fn in_game_note(&self) -> InGameNoteFormat {
        match self {
            Identifier::Spell(id) => InGameNoteFormat(format!("{{spell:{id}}}")),
            Identifier::Icon(_, file_data_id) => {
                InGameNoteFormat(format!("{{icon:{file_data_id}}}"))
            }
            Identifier::Marker(marker) => InGameNoteFormat(format!("{{rt{}}}", *marker as u8)),
            Identifier::Text(text) => InGameNoteFormat(text.clone()),
        }
    }
}
