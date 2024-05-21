use ordered_float::NotNan;

use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};

pub fn spells() -> Lookup<Spell> {
    [
        Spell {
            name: "Revival".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(115310),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(2, 45),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("48f8a538-33fd-48e6-8e16-1bafe9c7fe41"),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Invoke Yu'lon 1m".to_string(),
            icon_text: Some("1m".to_string()),
            identifier: Identifier::Spell(322118),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(1, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("38aa07c2-6628-47f6-8bba-a558698676a9"),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Invoke Yu'lon ~3m".to_string(),
            icon_text: Some("3m".to_string()),
            identifier: Identifier::Spell(322118),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(2, 30),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("60ec61e4-d8f4-4b28-82c4-5c4511018122"),
            enabled: false,
            minor: false,
        },
        Spell {
            name: "Invoke Chi-Ji 1m".to_string(),
            icon_text: Some("1m".to_string()),
            identifier: Identifier::Spell(325197),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(1, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("9622b096-bbeb-40c4-879e-a2c392403cda"),
            enabled: false,
            minor: false,
        },
        Spell {
            name: "Invoke Chi-Ji ~3m".to_string(),
            icon_text: Some("3m".to_string()),
            identifier: Identifier::Spell(325197),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(2, 30),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("05912352-ef9f-42cb-8ce3-48d91ab4dc6c"),
            enabled: true,
            minor: false,
        },
    ]
    .into_iter()
    .collect()
}
