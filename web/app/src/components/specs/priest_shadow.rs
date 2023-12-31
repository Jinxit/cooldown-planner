use crate::api::ui_spell::UiSpell;
use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};
use ordered_float::NotNan;
use std::time::Duration;
use uuid::uuid;

pub fn spells() -> Lookup<UiSpell> {
    [UiSpell {
        name: "Vampiric Embrace".to_string(),
        icon_text: None,
        identifier: Identifier::Spell(15286),
        power: NotNan::new(0.5).unwrap(),
        charges: 1,
        cooldown: TimeStep::mm_ss(2, 0),
        cast_time: TimeStep::mm_ss(0, 1),
        exclusive_with: Default::default(),
        uuid: SpellUuid::new("80594828-1888-46f9-9763-fe421c93e58d"),
        enabled: true,
        minor: true,
    }]
    .into_iter()
    .collect()
}
