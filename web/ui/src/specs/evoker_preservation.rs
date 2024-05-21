use ordered_float::NotNan;

use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};

pub fn spells() -> Lookup<Spell> {
    [
        Spell {
            name: "Stasis".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(370537),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(1, 30),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("1a72ab43-fedc-4711-b067-139aae294a9b"),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Dream Flight".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(359816),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(2, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("e1415788-f257-4bf0-abea-2a60867de272"),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Rewind 3m".to_string(),
            icon_text: Some("3m".to_string()),
            identifier: Identifier::Spell(363534),
            power: NotNan::new(1.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(3, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("275ac361-4f7a-4ebd-8c8f-76a1f4d52225"),
            enabled: true,
            minor: false,
        },
        Spell {
            name: "Rewind 4m".to_string(),
            icon_text: Some("4m".to_string()),
            identifier: Identifier::Spell(363534),
            // "Rewind has 2 charges, but its healing is reduced by 50%."
            // Divide by 4 and multiply by 3 to normalize to same strength as Rewind 3m,
            // then multiply by 0.5 to reduce strength by 50%
            power: NotNan::new(0.5 * (3.0 / 4.0)).unwrap(),
            charges: 2,
            cooldown: TimeStep::mm_ss(4, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("a4cbc956-603d-48f1-8a62-b0d9b390842b"),
            enabled: true,
            minor: false,
        },
    ]
    .into_iter()
    .collect()
}
