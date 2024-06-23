use ordered_float::NotNan;

use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};
use uuid::uuid;

pub fn spells() -> Lookup<Spell> {
    [
        Spell {
            name: "Tranquility 3m".to_string(),
            icon_text: Some("3m".to_string()),
            identifier: Identifier::Spell(740),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(3, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new(uuid!("43d4698a-6d7a-4afe-bc21-5b175988e5e0")),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Tranquility 2m".to_string(),
            icon_text: Some("2m".to_string()),
            identifier: Identifier::Spell(740),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(2, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new(uuid!("c3f8a190-2701-4bf7-87af-91ffb5ca969b")),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Convoke the Spirits".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(323764),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(1, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: [Identifier::Spell(33891)].into_iter().collect(),
            uuid: SpellUuid::new(uuid!("b8603366-ab57-413e-b6a9-a3c37af87a1c")),
            enabled: false,
            minor: false,
        },
        Spell {
            name: "Incarnation: Tree of Life".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(33891),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(3, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: [Identifier::Spell(323764)].into_iter().collect(),
            uuid: SpellUuid::new(uuid!("e561b6ff-077b-4ae5-97c7-395eab1062ef")),
            enabled: false,
            minor: false,
        },
        Spell {
            name: "Flourish".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(197721),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(1, 30),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new(uuid!("876d7838-af9e-4e05-b012-60c2655d539d")),
            enabled: false,
            minor: false,
        },
        Spell {
            name: "Stampeding Roar".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(288826),
            power: NotNan::new(0.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(1, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new(uuid!("e6f6d9c2-0d51-463c-ac51-52403c678a00")),
            enabled: false,
            minor: true,
        },
    ]
    .into_iter()
    .collect()
}
