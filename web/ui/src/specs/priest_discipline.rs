use ordered_float::NotNan;

use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};

pub fn spells() -> Lookup<Spell> {
    [
        Spell {
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
        Spell {
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
