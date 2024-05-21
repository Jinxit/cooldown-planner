use ordered_float::NotNan;

use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};

pub fn spells() -> Lookup<Spell> {
    [
        Spell {
            name: "Anti-Magic Zone".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(51052),
            power: NotNan::new(0.5).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(2, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("8052a299-9964-416a-8571-aba502db711d"),
            enabled: true,
            minor: true,
        },
        Spell {
            name: "Abomination Limb".to_string(),
            icon_text: None,
            identifier: Identifier::Spell(315443),
            power: NotNan::new(0.0).unwrap(),
            charges: 1,
            cooldown: TimeStep::mm_ss(2, 0),
            cast_time: TimeStep::mm_ss(0, 1),
            exclusive_with: Default::default(),
            uuid: SpellUuid::new("f6207491-bce5-46db-928d-2bd3a98de9a0"),
            enabled: false,
            minor: true,
        },
    ]
    .into_iter()
    .collect()
}
