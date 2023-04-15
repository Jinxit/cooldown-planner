use crate::api::ui_spell::UiSpell;
use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};
use ordered_float::NotNan;
use std::time::Duration;
use uuid::uuid;

pub fn spells() -> Lookup<UiSpell> {
    [
        UiSpell {
            name: "Power Word: Barrier".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(62618),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(3, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("fa224fb1-b7bb-4ae9-b527-2f328b10963f"),
            enabled: true,
            minor: false,
        },
        UiSpell {
            name: "Evangelism".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(246287),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(1, 30),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("b95796a0-ae3d-42f1-aa28-c5d1d35ddbee"),
            enabled: true,
            minor: false,
        },
    ]
    .into_iter()
    .collect()
}
