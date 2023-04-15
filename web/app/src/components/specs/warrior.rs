use crate::api::ui_spell::UiSpell;
use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};
use ordered_float::NotNan;
use std::time::Duration;
use uuid::uuid;

pub fn spells() -> Lookup<UiSpell> {
    [UiSpell {
        name: "Rallying Cry".to_string(),
        icon_text: None,
        identifier: Identifier::Spell(97462),
        power: NotNan::new(0.5).unwrap(),
        charges: 1,
        cooldown: TimeStep::mm_ss(3, 0),
        cast_time: TimeStep::mm_ss(0, 1),
        exclusive_with: Default::default(),
        uuid: SpellUuid::new("717c2202-9d5c-4c11-ac7c-0e24bf173aec"),
        enabled: true,
        minor: true,
    }]
    .into_iter()
    .collect()
}
