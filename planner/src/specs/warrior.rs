use ordered_float::NotNan;

use fight_domain::{FromMinutesSeconds, Identifier, Lookup, Spell, SpellUuid, TimeStep};
use uuid::uuid;

pub fn spells() -> Lookup<Spell> {
    [Spell {
        name: "Rallying Cry".to_string(),
        icon_text: None,
        identifier: Identifier::Spell(97462),
        power: NotNan::new(0.5).unwrap(),
        charges: 1,
        cooldown: TimeStep::mm_ss(3, 0),
        cast_time: TimeStep::mm_ss(0, 1),
        exclusive_with: Default::default(),
        uuid: SpellUuid::new(uuid!("717c2202-9d5c-4c11-ac7c-0e24bf173aec")),
        enabled: true,
        minor: true,
    }]
    .into_iter()
    .collect()
}
