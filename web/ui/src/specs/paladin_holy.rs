use ordered_float::NotNan;

use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};

pub fn spells() -> Lookup<Spell> {
    [
        Spell {
            name: "Aura Mastery".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(31821),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(3, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("06f9702f-af0e-4732-9045-6644668d3d75"),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Avenging Wrath".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(31884),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(2, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("e08621bd-333a-4931-afa4-930ffb9e0e1a"),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Divine Toll".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(304971),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(1, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("ca932f28-9e66-418e-90c4-64e96b019cf8"),
            enabled: false,
            minor: false,
        },
    ]
    .into_iter()
    .collect()
}
