use ordered_float::NotNan;

use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};

pub fn spells() -> Lookup<Spell> {
    [
        Spell {
            name: "Healing Tide Totem".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(108280),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(3, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("cfc494b5-92ec-48d2-96b1-cf98993a8f14"),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Spirit Link Totem".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(98008),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(3, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("19d5fcbc-1cb7-48db-852e-202c23a30a0e"),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Ascendance".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(114052),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(3, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("d0944411-372f-4c77-8ba8-bc49ffe1ed7d"),
            enabled: true,
            minor: false,
        },
    ]
    .into_iter()
    .collect()
}
