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
use tracing::warn;
use uuid::uuid;

pub fn mythic(
    instance_info: &JournalInstanceResponse,
    encounter_info: &JournalEncounterResponse,
) -> UiFight {
    UiFight::new(
        instance_info,
        encounter_info,
        Difficulty::Mythic,
        |props| view! { <AmalgamationParameters props/> },
        amalgamation_attacks,
        "boss/amalgamation.png",
        60,
    )
}

#[derive(Debug, Copy, Clone)]
struct AmalgamationProps {
    phase_1_clear_1_timer: RwSignal<Duration>,
    phase_1_clear_2_timer: RwSignal<Duration>,
    phase_1_clear_3_timer: RwSignal<Duration>,
    phase_2_start_timer: RwSignal<Duration>,
}

impl FightProps for AmalgamationProps {
    fn new() -> Self {
        Self {
            phase_1_clear_1_timer: RwSignal::new(Duration::mm_ss(0, 45)),
            phase_1_clear_2_timer: RwSignal::new(Duration::mm_ss(1, 25)),
            phase_1_clear_3_timer: RwSignal::new(Duration::mm_ss(2, 0)),
            phase_2_start_timer: RwSignal::new(Duration::mm_ss(2, 15)),
        }
    }
}

#[component]
fn AmalgamationParameters(props: AmalgamationProps) -> impl IntoView {
    view! {
        <div class="flex h-full w-full flex-wrap content-start gap-2 overflow-hidden p-2 transition-all">
            <h1 class="block border-r-2 border-r-slate-400 pr-2 my-auto py-2">"Phase 1"</h1>
            <TimerInput
                label="Clear 1"
                initial_value=props.phase_1_clear_1_timer.get_untracked()
                set_value=props.phase_1_clear_1_timer.write_only()
                max_value=props.phase_1_clear_2_timer.read_only()
            />
            <TimerInput
                label="Clear 2"
                initial_value=props.phase_1_clear_2_timer.get_untracked()
                set_value=props.phase_1_clear_2_timer.write_only()
                min_value=props.phase_1_clear_1_timer.read_only()
                max_value=props.phase_1_clear_3_timer.read_only()
            />
            <TimerInput
                label="Clear 3"
                initial_value=props.phase_1_clear_3_timer.get_untracked()
                set_value=props.phase_1_clear_3_timer.write_only()
                min_value=props.phase_1_clear_2_timer.read_only()
                max_value=props.phase_2_start_timer.read_only()
            />
            <div class="basis-full h-0"></div>
            <h1 class="block border-r-2 border-r-slate-400 pr-2 my-auto py-2">"Phase 2"</h1>
            <TimerInput
                label="Start"
                initial_value=props.phase_2_start_timer.get_untracked()
                set_value=props.phase_2_start_timer.write_only()
                min_value=props.phase_1_clear_3_timer.read_only()
            />
        </div>
    }
}

fn amalgamation_attacks(props: AmalgamationProps) -> Signal<Lookup<Attack>> {
    let phase_1_clear_1_timer = props.phase_1_clear_1_timer.memo();
    let phase_1_clear_2_timer = props.phase_1_clear_2_timer.memo();
    let phase_1_clear_3_timer = props.phase_1_clear_3_timer.memo();
    let phase_2_start_timer = props.phase_2_start_timer.memo();
    Signal::derive(move || {
        let on_pull = Attack {
            uuid: AttackUuid::new(uuid!("c408b8bb-f68f-44bb-8778-97f504aad8c0")),
            name: "On Pull".to_string(),
            power: NotNan::zero(),
            r#type: AttackType::Generic,
            timer: AttackTimer {
                dynamic_timer: Some(TimeStep::mm_ss(0, 3)),
                ..Default::default()
            },
        };

        let void = move |timer: TimeStep, uuid1, uuid2, uuid3| {
            vec![
                Attack {
                    uuid: uuid1,
                    name: "Void Run Away".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::Movement,
                    timer: AttackTimer {
                        dynamic_timer: Some(timer),
                        phase_end: Some(phase_2_start_timer.get().into()),
                        ..Default::default()
                    },
                },
                Attack {
                    uuid: uuid2,
                    name: "Void Damage".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::RaidDamage,
                    timer: AttackTimer {
                        dynamic_timer: Some(timer + TimeStep::mm_ss(0, 4)),
                        phase_end: Some(phase_2_start_timer.get().into()),
                        ..Default::default()
                    },
                },
                Attack {
                    uuid: uuid3,
                    name: "Fire Meteor Soak".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::RaidDamageStacked,
                    timer: AttackTimer {
                        dynamic_timer: Some(timer + TimeStep::mm_ss(0, 6)),
                        phase_end: Some(phase_2_start_timer.get().into()),
                        ..Default::default()
                    },
                },
            ]
        };

        let clear_debuff = |timer, uuid| Attack {
            uuid,
            name: "Clear Debuffs".to_string(),
            power: NotNan::one(),
            r#type: AttackType::Debuffs,
            timer: AttackTimer {
                dynamic_timer: Some(timer),
                phase_end: Some(phase_2_start_timer.get().into()),
                ..Default::default()
            },
        };

        let phase_1 = [
            vec![
                on_pull,
                clear_debuff(
                    phase_1_clear_1_timer.get().into(),
                    AttackUuid::new(uuid!("b896600e-9a84-480e-9da2-b1b6a12c3a8b")),
                ),
                clear_debuff(
                    phase_1_clear_2_timer.get().into(),
                    AttackUuid::new(uuid!("7bf4a86b-bdc4-4fb3-af58-a162d1710233")),
                ),
                clear_debuff(
                    phase_1_clear_3_timer.get().into(),
                    AttackUuid::new(uuid!("4ca7ee47-780c-40d8-a62a-17ac282b021e")),
                ),
            ],
            void(
                TimeStep::mm_ss(0, 34),
                AttackUuid::new(uuid!("e8b5472f-4741-448b-a7d2-23f5016d5e8c")),
                AttackUuid::new(uuid!("f04aea62-d8c6-40f5-b0ad-54a32abc1646")),
                AttackUuid::new(uuid!("8182440a-eb82-48ca-bc06-fa9397e08a90")),
            ),
            void(
                TimeStep::mm_ss(1, 10),
                AttackUuid::new(uuid!("e0402f86-31c9-4ab0-92d8-4883c88b6dd4")),
                AttackUuid::new(uuid!("e51b4863-a444-435c-ab9c-27769661c59b")),
                AttackUuid::new(uuid!("74d4d528-6809-4aff-bd93-292020e0f71f")),
            ),
            void(
                TimeStep::mm_ss(1, 45),
                AttackUuid::new(uuid!("c42f1a4f-3f22-4601-a469-bc870d9e891e")),
                AttackUuid::new(uuid!("82a75991-1fc6-4cec-b3fa-88e551e1aef0")),
                AttackUuid::new(uuid!("323a0166-1b43-4faa-8321-12d1c3e1f0dc")),
            ),
            void(
                TimeStep::mm_ss(2, 21),
                AttackUuid::new(uuid!("4fbbd0ca-92c1-47f3-ad2f-0fa0fad785a6")),
                AttackUuid::new(uuid!("56baf345-68fa-4f00-85d9-0b0a72bf316c")),
                AttackUuid::new(uuid!("39875925-f3c4-4a84-9195-430a747cbdb1")),
            ),
        ]
        .into_iter()
        .flatten()
        .filter(|a| a.timer.static_timer() <= phase_2_start_timer.get().into());

        let intermission = Attack {
            uuid: AttackUuid::new(uuid!("3a31d00d-a262-44dc-beb1-de073a245eba")),
            name: "Intermission".to_string(),
            power: NotNan::zero(),
            r#type: AttackType::Generic,
            timer: AttackTimer {
                dynamic_timer: Some(phase_2_start_timer.get().into()),
                ..Default::default()
            },
        };

        let phase_2_base_timer = AttackTimer {
            phase_start: Some(phase_2_start_timer.get().into()),
            dynamic_trigger_cleu_event: Some(CleuEvent {
                r#type: CleuEventType::SpellCastSuccess,
                event: 406730,
                counter: 1,
            }),
            ..Default::default()
        };

        let phase_2_start = Attack {
            uuid: AttackUuid::new(uuid!("90459fb3-cd18-4484-b815-df360ae49a18")),
            name: "Phase 2".to_string(),
            power: NotNan::zero(),
            r#type: AttackType::Generic,
            timer: AttackTimer {
                dynamic_timer: Some(TimeStep::mm_ss(0, 14)),
                ..phase_2_base_timer.clone()
            },
        };

        let mythic_debuff_soaks = |timer, uuid1, uuid2| {
            vec![
                Attack {
                    uuid: uuid1,
                    name: "Mythic Debuff".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::RaidDamage,
                    timer: AttackTimer {
                        dynamic_timer: Some(timer),
                        ..phase_2_base_timer.clone()
                    },
                },
                Attack {
                    uuid: uuid2,
                    name: "Soaks".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::Movement,
                    timer: AttackTimer {
                        dynamic_timer: Some(timer + TimeStep::mm_ss(0, 3)),
                        ..phase_2_base_timer.clone()
                    },
                },
            ]
        };

        let meteor_soak_run_pull = |timer, uuid1, uuid2| {
            vec![
                Attack {
                    uuid: uuid1,
                    name: "Meteor Soak + Run".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::RaidDamageStacked,
                    timer: AttackTimer {
                        dynamic_timer: Some(timer),
                        ..phase_2_base_timer.clone()
                    },
                },
                Attack {
                    uuid: uuid2,
                    name: "Pull AoE".to_string(),
                    power: NotNan::one(),
                    r#type: AttackType::RaidDamage,
                    timer: AttackTimer {
                        dynamic_timer: Some(timer + TimeStep::mm_ss(0, 6)),
                        ..phase_2_base_timer.clone()
                    },
                },
            ]
        };

        let phase_2 = [
            vec![intermission, phase_2_start],
            mythic_debuff_soaks(
                TimeStep::mm_ss(0, 34),
                AttackUuid::new(uuid!("58f8aee5-d29b-46cd-bbf2-7d404f8f8286")),
                AttackUuid::new(uuid!("a0ca74cd-602d-41cc-8ce0-bf22b15b5cf4")),
            ),
            meteor_soak_run_pull(
                TimeStep::mm_ss(0, 55),
                AttackUuid::new(uuid!("164a3a75-8c9c-4195-a541-83564d6002e3")),
                AttackUuid::new(uuid!("1a41708c-cbfd-44a9-b974-552f99d795f3")),
            ),
            mythic_debuff_soaks(
                TimeStep::mm_ss(1, 26),
                AttackUuid::new(uuid!("62071d69-e5c0-4bac-9fdb-b48a4b7e58b9")),
                AttackUuid::new(uuid!("798da751-26f6-405b-9beb-b3a1f2d6c0d6")),
            ),
            meteor_soak_run_pull(
                TimeStep::mm_ss(1, 44),
                AttackUuid::new(uuid!("e40c7ef5-6370-47bc-a5ea-0f4217d41644")),
                AttackUuid::new(uuid!("e83b6e3d-c877-4396-99d5-2520cfbf33f3")),
            ),
            mythic_debuff_soaks(
                TimeStep::mm_ss(2, 14),
                AttackUuid::new(uuid!("c82d8f4b-8ea1-4723-9f59-6c887a4e3d94")),
                AttackUuid::new(uuid!("09362fb9-b528-4303-a113-5537e9215eb3")),
            ),
            meteor_soak_run_pull(
                TimeStep::mm_ss(2, 32),
                AttackUuid::new(uuid!("d6222425-cb07-4c74-b4d5-b08a6d63e70b")),
                AttackUuid::new(uuid!("81b44ff0-80b0-43c0-b78b-08d873725815")),
            ),
            mythic_debuff_soaks(
                TimeStep::mm_ss(3, 1),
                AttackUuid::new(uuid!("d8fc6543-2329-4df6-87ca-8b1a995b37e2")),
                AttackUuid::new(uuid!("a90b2150-fea5-45dc-909f-66d4c419bd2a")),
            ),
            meteor_soak_run_pull(
                TimeStep::mm_ss(3, 19),
                AttackUuid::new(uuid!("79f788e1-f5c7-4823-b48b-c8312dc6a8a4")),
                AttackUuid::new(uuid!("ae5ce90d-cbac-4edc-bdbe-d066ddecdf97")),
            ),
            mythic_debuff_soaks(
                TimeStep::mm_ss(3, 50),
                AttackUuid::new(uuid!("9ab1a047-d42a-402b-8607-306569f49bb0")),
                AttackUuid::new(uuid!("af52f39f-0f20-4439-b582-49ab47714416")),
            ),
            meteor_soak_run_pull(
                TimeStep::mm_ss(4, 8),
                AttackUuid::new(uuid!("eb227440-6c62-466e-8ca1-f975c24b43bb")),
                AttackUuid::new(uuid!("af55b93e-d4c2-45e0-9b21-721fbe8070f5")),
            ),
            mythic_debuff_soaks(
                TimeStep::mm_ss(4, 39),
                AttackUuid::new(uuid!("efe17196-0d82-4906-9c23-c6c595cada4c")),
                AttackUuid::new(uuid!("1af5126d-0545-48b0-a63d-0d2b76270c95")),
            ),
            meteor_soak_run_pull(
                TimeStep::mm_ss(4, 57),
                AttackUuid::new(uuid!("ed8ca59b-2f56-463b-8ed1-39dad430263f")),
                AttackUuid::new(uuid!("87727c6c-53b6-40ff-8942-1a928cd94bec")),
            ),
        ]
        .into_iter()
        .flatten();

        phase_1
            .chain(phase_2)
            .sorted_by_key(|attack| attack.timer.static_timer())
            .collect::<Lookup<Attack>>()
    })
}
