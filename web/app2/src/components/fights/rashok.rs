use crate::api::ui_fight::{FightProps, UiFight};
use crate::components::fights::Difficulty;
use crate::components::TimerInput;
use crate::reactive::memo::Memoize;
use auto_battle_net::game_data::journal::journal_encounter::JournalEncounterResponse;
use auto_battle_net::game_data::journal::journal_instance::JournalInstanceResponse;
use fight_domain::{
    Attack, AttackTimer, AttackType, AttackUuid, CleuEvent, CleuEventType, FromMinutesSeconds,
    Lookup, TimeStep,
};
use itertools::Itertools;
use leptos::*;
use num_traits::{One, Zero};
use ordered_float::NotNan;
use std::time::Duration;
use uuid::uuid;

pub fn mythic(
    instance_info: &JournalInstanceResponse,
    encounter_info: &JournalEncounterResponse,
) -> UiFight {
    UiFight::new(
        instance_info,
        encounter_info,
        Difficulty::Mythic,
        |props| view! { () },
        rashok_attacks,
        "boss/rashok.png",
        29,
    )
}

#[component]
fn RashokParameters(_props: ()) -> impl IntoView {
    view! {
        <div class="flex h-full w-full flex-wrap content-start gap-2 overflow-hidden p-2 transition-all"></div>
    }
}

fn rashok_attacks(_props: ()) -> Signal<Lookup<Attack>> {
    Signal::derive(move || {
        let dynamic_trigger = move |phase: u64| {
            (phase > 0).then_some(CleuEvent {
                r#type: CleuEventType::SpellAuraApplied,
                event: 401419,
                counter: phase,
            })
        };

        let intermission_end =
            move |timer: TimeStep, phase: u64, phase_start: Option<Duration>, uuid| {
                vec![Attack {
                    uuid,
                    name: if phase == 0 {
                        "On Pull".to_string()
                    } else {
                        "Intermission End".to_string()
                    },
                    power: NotNan::zero(),
                    r#type: AttackType::Generic,
                    timer: AttackTimer {
                        phase_start: phase_start.map(Into::into),
                        dynamic_timer: Some(timer),
                        dynamic_trigger_cleu_event: dynamic_trigger(phase),
                        ..Default::default()
                    },
                }]
            };

        let jump_aoe =
            move |timer: TimeStep, damage: u32, phase: u64, phase_start: Option<Duration>, uuid| {
                vec![Attack {
                    uuid,
                    name: format!("Jump AoE {damage}k"),
                    power: NotNan::new((damage as f64) / 200.0).unwrap(),
                    r#type: AttackType::RaidDamage,
                    timer: AttackTimer {
                        phase_start: phase_start.map(Into::into),
                        dynamic_timer: Some(timer),
                        dynamic_trigger_cleu_event: dynamic_trigger(phase),
                        ..Default::default()
                    },
                }]
            };

        let heal_absorb =
            move |timer: TimeStep, phase: u64, phase_start: Option<Duration>, uuid| {
                vec![Attack {
                    uuid,
                    name: "Heal Absorb".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::RaidDamage,
                    timer: AttackTimer {
                        phase_start: phase_start.map(Into::into),
                        dynamic_timer: Some(timer),
                        dynamic_trigger_cleu_event: dynamic_trigger(phase),
                        ..Default::default()
                    },
                }]
            };

        let meteor_soak =
            move |timer: TimeStep, phase: u64, phase_start: Option<Duration>, uuid1, uuid2| {
                let mut attacks = vec![];
                attacks.push(Attack {
                    uuid: uuid1,
                    name: format!(
                        "Meteor Soak{}",
                        (phase < 2).then_some(" + Clears").unwrap_or_default()
                    ),
                    power: NotNan::one(),
                    r#type: AttackType::RaidDamageStacked,
                    timer: AttackTimer {
                        phase_start: phase_start.map(Into::into),
                        dynamic_timer: Some(timer),
                        dynamic_trigger_cleu_event: dynamic_trigger(phase),
                        ..Default::default()
                    },
                });
                if phase < 2 {
                    attacks.push(Attack {
                        uuid: uuid2,
                        name: "Clears pt 2".to_string(),
                        power: NotNan::one(),
                        r#type: AttackType::RaidDamageStacked,
                        timer: AttackTimer {
                            phase_start: phase_start.map(Into::into),
                            dynamic_timer: Some(timer + TimeStep::mm_ss(0, 4)),
                            dynamic_trigger_cleu_event: dynamic_trigger(phase),
                            ..Default::default()
                        },
                    });
                }
                attacks
            };

        let soaks =
            move |timer: TimeStep, phase: u64, phase_start: Option<Duration>, uuid1, uuid2| {
                vec![
                    Attack {
                        uuid: uuid1,
                        name: "Soaks Run".to_string(),
                        power: NotNan::one(),
                        r#type: AttackType::Movement,
                        timer: AttackTimer {
                            phase_start: phase_start.map(Into::into),
                            dynamic_timer: Some(timer),
                            dynamic_trigger_cleu_event: dynamic_trigger(phase),
                            ..Default::default()
                        },
                    },
                    Attack {
                        uuid: uuid2,
                        name: "Soaks".to_string(),
                        power: NotNan::one(),
                        r#type: AttackType::RaidDamage,
                        timer: AttackTimer {
                            phase_start: phase_start.map(Into::into),
                            dynamic_timer: Some(timer + TimeStep::mm_ss(0, 8)),
                            dynamic_trigger_cleu_event: dynamic_trigger(phase),
                            ..Default::default()
                        },
                    },
                ]
            };

        let frontal_bait =
            move |timer: TimeStep, phase: u64, phase_start: Option<Duration>, uuid| {
                vec![Attack {
                    uuid,
                    name: "Frontal Bait".to_string(),
                    power: NotNan::zero(),
                    r#type: AttackType::Generic,
                    timer: AttackTimer {
                        phase_start: phase_start.map(Into::into),
                        dynamic_timer: Some(timer),
                        dynamic_trigger_cleu_event: dynamic_trigger(phase),
                        ..Default::default()
                    },
                }]
            };

        let intermission = move |timer: TimeStep,
                                 phase: u64,
                                 phase_start: Option<Duration>,
                                 uuid1,
                                 uuid2,
                                 uuid3,
                                 uuid4| {
            if phase == 2 {
                return vec![];
            }
            vec![
                Attack {
                    uuid: uuid1,
                    name: "Intermission 0/20 sec".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::RotDamage,
                    timer: AttackTimer {
                        phase_start: phase_start.map(Into::into),
                        dynamic_timer: Some(timer),
                        dynamic_trigger_cleu_event: dynamic_trigger(phase),
                        ..Default::default()
                    },
                },
                Attack {
                    uuid: uuid2,
                    name: "Intermission 5/20 sec".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::RotDamage,
                    timer: AttackTimer {
                        phase_start: phase_start.map(Into::into),
                        dynamic_timer: Some(timer + TimeStep::mm_ss(0, 5)),
                        dynamic_trigger_cleu_event: dynamic_trigger(phase),
                        ..Default::default()
                    },
                },
                Attack {
                    uuid: uuid3,
                    name: "Intermission 10/20 sec".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::RotDamage,
                    timer: AttackTimer {
                        phase_start: phase_start.map(Into::into),
                        dynamic_timer: Some(timer + TimeStep::mm_ss(0, 10)),
                        dynamic_trigger_cleu_event: dynamic_trigger(phase),
                        ..Default::default()
                    },
                },
                Attack {
                    uuid: uuid4,
                    name: "Intermission 15/20 sec".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::RotDamage,
                    timer: AttackTimer {
                        phase_start: phase_start.map(Into::into),
                        dynamic_timer: Some(timer + TimeStep::mm_ss(0, 15)),
                        dynamic_trigger_cleu_event: dynamic_trigger(phase),
                        ..Default::default()
                    },
                },
            ]
        };

        let phase = move |phase_start: Option<Duration>,
                          phase: u64,
                          damage1,
                          damage2,
                          damage3,
                          timers: Vec<(TimeStep, AttackUuid)>| {
            [
                intermission_end(timers[0].0, phase, phase_start, timers[0].1),
                jump_aoe(timers[1].0, damage1, phase, phase_start, timers[1].1),
                heal_absorb(timers[2].0, phase, phase_start, timers[2].1),
                meteor_soak(timers[3].0, phase, phase_start, timers[3].1, timers[4].1),
                soaks(timers[5].0, phase, phase_start, timers[5].1, timers[6].1),
                jump_aoe(timers[7].0, damage2, phase, phase_start, timers[7].1),
                heal_absorb(timers[8].0, phase, phase_start, timers[8].1),
                meteor_soak(timers[9].0, phase, phase_start, timers[9].1, timers[10].1),
                jump_aoe(timers[11].0, damage3, phase, phase_start, timers[11].1),
                frontal_bait(timers[12].0, phase, phase_start, timers[12].1),
                heal_absorb(timers[13].0, phase, phase_start, timers[13].1),
                intermission(
                    timers[14].0,
                    phase,
                    phase_start,
                    timers[14].1,
                    timers[15].1,
                    timers[16].1,
                    timers[17].1,
                ),
            ]
            .into_iter()
            .flatten()
        };

        let phase_0 = phase(
            None,
            0,
            245,
            295,
            340,
            vec![
                (
                    TimeStep::mm_ss(0, 3),
                    AttackUuid::new(uuid!("48788568-2cf6-42fd-a0a6-65bcbedc8db1")),
                ),
                (
                    TimeStep::mm_ss(0, 14),
                    AttackUuid::new(uuid!("b34ecb1d-3d12-41c4-8c96-32dfd276d37d")),
                ),
                (
                    TimeStep::mm_ss(0, 23),
                    AttackUuid::new(uuid!("05d153ac-0f5a-4e9c-8b73-0df0c0e5430d")),
                ),
                (
                    TimeStep::mm_ss(0, 26),
                    AttackUuid::new(uuid!("6064b938-0437-4ffb-9eff-0071ca505851")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("5ad36fb9-6f02-46ee-ba5c-e015be5fa61d")),
                ),
                (
                    TimeStep::mm_ss(0, 41),
                    AttackUuid::new(uuid!("fd6da248-3c13-4d08-b846-b7a85dd15740")),
                ),
                (
                    TimeStep::mm_ss(0, 49),
                    AttackUuid::new(uuid!("c2f3209a-5a6c-4d97-a8c7-5a2a89e02cd6")),
                ),
                (
                    TimeStep::mm_ss(0, 57),
                    AttackUuid::new(uuid!("b6f020c8-9801-4e19-9e57-844105da8dda")),
                ),
                (
                    TimeStep::mm_ss(1, 8),
                    AttackUuid::new(uuid!("4f7dd959-d6c9-49ee-9561-2fcfda627bff")),
                ),
                (
                    TimeStep::mm_ss(1, 9),
                    AttackUuid::new(uuid!("b0e4377b-2106-454e-ac31-fde248f781bf")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("1daa358c-47a8-4b7c-be8f-dacc5ee8edb3")),
                ),
                (
                    TimeStep::mm_ss(1, 30),
                    AttackUuid::new(uuid!("b27695df-ed78-4c3b-95b1-4091d9dfef25")),
                ),
                (
                    TimeStep::mm_ss(1, 38),
                    AttackUuid::new(uuid!("7afb05c4-705a-4c81-9d8c-bb894b1667e3")),
                ),
                (
                    TimeStep::mm_ss(1, 42),
                    AttackUuid::new(uuid!("f83fecbf-3e7f-4e6a-b989-953e62488756")),
                ),
                (
                    TimeStep::mm_ss(1, 52),
                    AttackUuid::new(uuid!("69476c1b-3b06-446f-8ed5-c99583bc82d2")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("616252a3-c3e1-43d7-b843-aa36569cff2e")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("4bfa08af-4485-4a6d-88d3-d423b797ff97")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("883eb85c-8df1-4b72-9d36-5bae687d6da1")),
                ),
            ],
        );
        let phase_1 = phase(
            Some(Duration::mm_ss(1, 52)),
            1,
            390,
            440,
            490,
            vec![
                (
                    TimeStep::mm_ss(0, 20),
                    AttackUuid::new(uuid!("68593620-2782-47d5-bbd8-528a3db58e9e")),
                ),
                (
                    TimeStep::mm_ss(0, 36),
                    AttackUuid::new(uuid!("fbb46d7d-a4a2-45a1-84c1-0c3e4f18fa33")),
                ),
                (
                    TimeStep::mm_ss(0, 44),
                    AttackUuid::new(uuid!("8aea5614-a47b-4ab6-b8d4-c4c18838e627")),
                ),
                (
                    TimeStep::mm_ss(0, 48),
                    AttackUuid::new(uuid!("ba1cb4de-5216-4ebb-9148-8d62a5bfdb7e")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("94422db5-0f96-4852-866d-a3e9578028c6")),
                ),
                (
                    TimeStep::mm_ss(1, 4),
                    AttackUuid::new(uuid!("43746917-ee1e-498e-aeb8-97fa9246a5b0")),
                ),
                (
                    TimeStep::mm_ss(1, 12),
                    AttackUuid::new(uuid!("e7d4522d-e067-4fa1-9e4a-e2c19edd9be1")),
                ),
                (
                    TimeStep::mm_ss(1, 19),
                    AttackUuid::new(uuid!("1465b10b-5f8a-49d9-9a8b-d4ccbe14e90a")),
                ),
                (
                    TimeStep::mm_ss(1, 30),
                    AttackUuid::new(uuid!("b793a531-db29-4bcd-a6c7-fe343961ae35")),
                ),
                (
                    TimeStep::mm_ss(1, 35),
                    AttackUuid::new(uuid!("2023c45a-9748-4ed3-a49c-74feedf92062")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("ef75e175-0df4-46b4-9f65-9dc8226f4e52")),
                ),
                (
                    TimeStep::mm_ss(1, 52),
                    AttackUuid::new(uuid!("eb4abfed-0099-4da2-a33f-5a9e2e1befa1")),
                ),
                (
                    TimeStep::mm_ss(2, 2),
                    AttackUuid::new(uuid!("142c372f-c7f6-4b69-a09e-c42be314911a")),
                ),
                (
                    TimeStep::mm_ss(2, 3),
                    AttackUuid::new(uuid!("3e358a6f-40f0-4be5-a36b-d11a18acb246")),
                ),
                (
                    TimeStep::mm_ss(2, 15),
                    AttackUuid::new(uuid!("c0a2f8a0-c3cb-48b4-8203-8fb24a61a846")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("017bc8cd-a93f-4a2b-b3f1-d5c6372159e2")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("8e1a5688-f95e-438d-b3bb-8ad1543828e1")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("bb89e2bd-69bb-4e7a-8900-f05bdf239c9a")),
                ),
            ],
        );
        let phase_2 = phase(
            Some(Duration::mm_ss(4, 7)),
            2,
            540,
            585,
            625,
            vec![
                (
                    TimeStep::mm_ss(0, 20),
                    AttackUuid::new(uuid!("1aea2cab-ef04-49a4-9ad1-8ca3574ca1cb")),
                ),
                (
                    TimeStep::mm_ss(0, 36),
                    AttackUuid::new(uuid!("52851734-f062-4ec9-9b81-9b78358573a8")),
                ),
                (
                    TimeStep::mm_ss(0, 44),
                    AttackUuid::new(uuid!("9442de54-ecb6-43dc-89eb-296630548237")),
                ),
                (
                    TimeStep::mm_ss(0, 48),
                    AttackUuid::new(uuid!("29eda0ee-d9d4-4b85-9603-3d1832d6e9da")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("9f7d42cf-278f-48cc-824d-b2fc259a502d")),
                ),
                (
                    TimeStep::mm_ss(1, 4),
                    AttackUuid::new(uuid!("6fb0eaeb-8339-4a10-9740-af219e773a12")),
                ),
                (
                    TimeStep::mm_ss(1, 12),
                    AttackUuid::new(uuid!("bfcba1ec-d708-4f3d-8352-c969ab722f09")),
                ),
                (
                    TimeStep::mm_ss(1, 19),
                    AttackUuid::new(uuid!("709dd4ed-9b52-4f3d-907d-57ddf63b3dc4")),
                ),
                (
                    TimeStep::mm_ss(1, 30),
                    AttackUuid::new(uuid!("5d5dc083-2888-40af-90ae-7b5c0748b5fa")),
                ),
                (
                    TimeStep::mm_ss(1, 35),
                    AttackUuid::new(uuid!("186d07fc-07ed-49d6-8c3c-d27822032b7b")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("6349fcea-3143-4bd3-8228-f6a00c461684")),
                ),
                (
                    TimeStep::mm_ss(1, 52),
                    AttackUuid::new(uuid!("55cf5ae2-1451-4d16-b817-277fa3d9a554")),
                ),
                (
                    TimeStep::mm_ss(2, 2),
                    AttackUuid::new(uuid!("01cd7b80-5187-4712-a1ad-c8a5d1bf92fe")),
                ),
                (
                    TimeStep::mm_ss(2, 3),
                    AttackUuid::new(uuid!("f73446ae-2469-4b2f-a2ab-9568ae761781")),
                ),
                (
                    TimeStep::mm_ss(2, 15),
                    AttackUuid::new(uuid!("c0a2f8a0-c3cb-48b4-8203-8fb24a61a846")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("017bc8cd-a93f-4a2b-b3f1-d5c6372159e2")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("8e1a5688-f95e-438d-b3bb-8ad1543828e1")),
                ),
                (
                    TimeStep::zero(),
                    AttackUuid::new(uuid!("bb89e2bd-69bb-4e7a-8900-f05bdf239c9a")),
                ),
            ],
        );

        let enrage = [Attack {
            uuid: AttackUuid::new(uuid!("82b56325-f7e5-4305-96d2-af0750f14515")),
            name: "Enrage".to_string(),
            power: NotNan::zero(),
            r#type: AttackType::Generic,
            timer: AttackTimer {
                phase_start: Some(TimeStep::mm_ss(4, 7)),
                dynamic_timer: Some(TimeStep::mm_ss(2, 23)),
                dynamic_trigger_cleu_event: dynamic_trigger(2),
                ..Default::default()
            },
        }]
        .into_iter();

        phase_0
            .chain(phase_1)
            .chain(phase_2)
            .chain(enrage)
            .sorted_by_key(|attack| attack.timer.static_timer())
            .collect::<Lookup<Attack>>()
    })
}
