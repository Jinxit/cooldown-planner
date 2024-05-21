use ordered_float::NotNan;

use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};

pub fn spells() -> Lookup<Spell> {
    [
        Spell {
            name: "Holy Word: Salvation".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(265202),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(5, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("ee38dcfe-55c5-4506-807b-228993e01fe9"),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Divine Hymn".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(64843),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(3, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("090e4af5-833a-4174-b360-f22721a1ff60"),
            enabled: true,
            minor: false,
        },
    ]
    .into_iter()
    .collect()
}
