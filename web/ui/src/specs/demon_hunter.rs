use ordered_float::NotNan;

use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};

pub fn spells() -> Lookup<Spell> {
    [Spell {
        name: "Darkness".to_string(),
        icon_text: None,
        identifier: Identifier::Spell(196718),
        power: NotNan::new(0.5).unwrap(),
        charges: 1,
        cooldown: TimeStep::mm_ss(3, 0),
        cast_time: TimeStep::mm_ss(0, 1),
        exclusive_with: Default::default(),
        uuid: SpellUuid::new("b6030ff4-5386-41ca-b653-a89a7a4fa39d"),
        enabled: true,
        minor: true,
    }]
    .into_iter()
    .collect()
}
