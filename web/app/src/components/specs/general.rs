use crate::api::ui_spell::UiSpell;
use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};
use ordered_float::NotNan;
use std::time::Duration;
use uuid::uuid;

pub fn spells() -> Lookup<UiSpell> {
    [
        UiSpell {
            name: "Personals".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(71),
            power: NotNan::new(0.5 * (1.0 / 60.0)).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(60, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("6ae80fcd-ccea-4159-b793-fc580bb757d7"),
            enabled: false,
            minor: true,
        },
        UiSpell {
            name: "Healthstone".to_string(),
            icon_text: None,
            identifier: Identifier::Icon("warlock_healthstone".to_string(), 538745),
            power: NotNan::new(0.25 * (1.0 / 60.0)).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(60, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("ac3f5e1b-4868-409c-9558-2c457ba9360b"),
            enabled: false,
            minor: true,
        },
        UiSpell {
            name: "Healing Potion".to_string(),
            icon_text: None,
            identifier: Identifier::Icon("inv_10_alchemy_bottle_shape4_red".to_string(), 4497595),
            power: NotNan::new(0.25 * (1.0 / 60.0)).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(60, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("a9c60839-4c4e-4c17-b23d-72f7d69df997"),
            enabled: false,
            minor: true,
        },
        UiSpell {
            name: "Fury Of The Aspects".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(390386),
            power: NotNan::new(0.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(10, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("5031d1c8-fbf9-4a79-b775-d82a00aca3ee"),
            enabled: false,
            minor: true,
        },
        UiSpell {
            name: "Gateway".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(111771),
            power: NotNan::new(0.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(2, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("36926c11-1844-422d-be04-cc2f09bcbf2f"),
            enabled: false,
            minor: true,
        },
    ]
    .into_iter()
    .collect()
}
