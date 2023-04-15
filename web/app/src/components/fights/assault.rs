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
use leptos::{component, IntoSignal, RwSignal, Signal, SignalGet};
use num_traits::{One, Zero};
use ordered_float::NotNan;
use std::time::Duration;

pub fn mythic(
    instance_info: &JournalInstanceResponse,
    encounter_info: &JournalEncounterResponse,
) -> UiFight {
    UiFight::new(
        instance_info,
        encounter_info,
        Difficulty::Mythic,
        |props| view! {  <AssaultParameters props/> },
        assault_attacks,
        "boss/assault.png",
        35,
    )
}

#[derive(Debug, Copy, Clone)]
struct AssaultProps {
    shield_durations: RwSignal<Duration>,
    phase_2_start_timer: RwSignal<Duration>,
}

impl FightProps for AssaultProps {
    fn new() -> Self {
        Self {
            shield_durations: create_rw_signal(Duration::mm_ss(0, 15)),
            phase_2_start_timer: create_rw_signal(Duration::mm_ss(4, 37)),
        }
    }
}

#[component]
fn AssaultParameters(props: AssaultProps) -> impl IntoView {
    view! {
        <div class="flex h-full w-full flex-wrap content-start gap-2 overflow-hidden p-2 transition-all">
            <TimerInput
                label="Shield Durations"
                initial_value=props.shield_durations.get_untracked()
                set_value=props.shield_durations.write_only()
            />
            <TimerInput
                label="Phase 2 Start"
                initial_value=props.phase_2_start_timer.get_untracked()
                set_value=props.phase_2_start_timer.write_only()
            />
        </div>
    }
}

fn assault_attacks(props: AssaultProps) -> Signal<Lookup<Attack>> {
    let shield_durations = props.shield_durations.memo();
    let phase_2_start_timer = props.phase_2_start_timer.memo();
    Signal::derive(move || {
        let on_pull = Attack {
            uuid: AttackUuid::new("e79bbdaf-fa90-4500-ba1a-1996b18a01d1"),
            name: "On Pull".to_string(),
            power: NotNan::zero(),
            r#type: AttackType::Generic,
            timer: AttackTimer {
                dynamic_timer: Some(TimeStep::mm_ss(0, 3)),
                ..Default::default()
            },
        };

        let aoe_shield = move |timer: TimeStep, uuid| Attack {
            uuid,
            name: "AoE Shield x 2".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RaidDamage,
            timer: AttackTimer {
                dynamic_timer: Some(timer),
                phase_end: Some(phase_2_start_timer.get().into()),
                ..Default::default()
            },
        };

        let aoe_shields = move |timer: TimeStep, uuid1, uuid2| {
            vec![
                aoe_shield(timer, uuid1),
                aoe_shield(timer + (shield_durations.get() / 2).into(), uuid2),
            ]
        };

        let debuffs = move |phase_start: TimeStep, side: Option<&str>, counter: u64, uuid| {
            vec![Attack {
                uuid,
                name: format!(
                    "Debuffs {}",
                    side.map(|s| format!("({s})")).unwrap_or_default()
                ),
                power: NotNan::one(),
                r#type: AttackType::Dispels,
                timer: AttackTimer {
                    dynamic_timer: Some(TimeStep::mm_ss(0, 8)),
                    phase_start: Some(phase_start),
                    phase_end: Some(phase_2_start_timer.get().into()),
                    dynamic_trigger_cleu_event: Some(CleuEvent {
                        r#type: CleuEventType::SpellAuraRemoved,
                        event: 397383,
                        counter,
                    }),
                },
            }]
        };

        let pushback = move |timer: TimeStep, side: &str, uuid| {
            vec![Attack {
                uuid,
                name: format!("Pushback ({side})"),
                power: NotNan::one(),
                r#type: AttackType::Movement,
                timer: AttackTimer {
                    dynamic_timer: Some(timer),
                    phase_end: Some(phase_2_start_timer.get().into()),
                    ..Default::default()
                },
            }]
        };

        let phase_1 = [
            vec![on_pull],
            aoe_shields(
                TimeStep::mm_ss(0, 29),
                AttackUuid::new("8ce1db8f-0aea-4a69-93c4-f4b4984f9334"),
                AttackUuid::new("eb861988-38ce-440a-a87d-c5c1625a5f8a"),
            ),
            debuffs(
                TimeStep::mm_ss(0, 37),
                None,
                2,
                AttackUuid::new("69581ed9-010a-424d-873a-7e317999a1ae"),
            ),
            pushback(
                TimeStep::mm_ss(1, 16),
                "Right",
                AttackUuid::new("86471a73-2a9a-4b2c-aca3-d45948fdcc9f"),
            ),
            aoe_shields(
                TimeStep::mm_ss(1, 25),
                AttackUuid::new("41990e8f-ee10-477d-a0c3-5f29a01eba86"),
                AttackUuid::new("1a525b15-9d62-4e6e-a10a-a0f185622de6"),
            ),
            debuffs(
                TimeStep::mm_ss(0, 37),
                Some("Right"),
                3,
                AttackUuid::new("a5b64f38-bfc9-416a-b475-457d20215c5b"),
            ),
            aoe_shields(
                TimeStep::mm_ss(2, 18),
                AttackUuid::new("199afa14-0b07-47e4-afd9-72bc0273062c"),
                AttackUuid::new("ff187594-f3e4-48aa-9617-f8ff28c22559"),
            ),
            pushback(
                TimeStep::mm_ss(2, 21),
                "Left",
                AttackUuid::new("934264dc-5260-4ab6-aa7c-b426544b1e20"),
            ),
            debuffs(
                TimeStep::mm_ss(2, 29),
                Some("Left"),
                4,
                AttackUuid::new("7a539f88-391b-43b6-9a8d-03c962cfe59d"),
            ),
            pushback(
                TimeStep::mm_ss(2, 55),
                "Right",
                AttackUuid::new("134ba25d-9f99-47bf-b0c5-6e3066a6e993"),
            ),
            aoe_shields(
                TimeStep::mm_ss(3, 5),
                AttackUuid::new("624817cd-40a6-44f7-b81f-f329a6516dee"),
                AttackUuid::new("bc41322c-c84c-42c2-a135-5cdf71d49910"),
            ),
            debuffs(
                TimeStep::mm_ss(3, 13),
                Some("Right"),
                5,
                AttackUuid::new("5f5cbb3f-bba0-41b6-a3e6-c7b5743d1e02"),
            ),
            aoe_shields(
                TimeStep::mm_ss(3, 58),
                AttackUuid::new("b9be5d57-f03f-468f-9b37-7e67513fb332"),
                AttackUuid::new("78595e3d-b10c-4231-98fa-491bb81868fd"),
            ),
            pushback(
                TimeStep::mm_ss(4, 3),
                "Left",
                AttackUuid::new("da6968fb-554c-4a66-a10e-fdf7779e2f1d"),
            ),
            debuffs(
                TimeStep::mm_ss(4, 9),
                Some("Left"),
                6,
                AttackUuid::new("44541e19-f99a-4f2e-9f3c-8a21fc9e2356"),
            ),
            pushback(
                TimeStep::mm_ss(4, 40),
                "Right",
                AttackUuid::new("a353094d-5ecd-4f6c-8a90-a9520e0f70a5"),
            ),
            aoe_shields(
                TimeStep::mm_ss(4, 45),
                AttackUuid::new("af0d323f-5ce2-4269-845b-a88d162d8c1a"),
                AttackUuid::new("9d4deea3-be0f-4dd8-a391-46b4da5e00ed"),
            ),
            debuffs(
                TimeStep::mm_ss(4, 53),
                Some("Right"),
                7,
                AttackUuid::new("6e26a40d-d110-4b24-a3ac-3e327c8f20a7"),
            ),
        ]
        .into_iter()
        .flatten()
        .filter(|a| a.timer.static_timer() <= phase_2_start_timer.get().into());

        let phase_2 = move |timer: TimeStep, uuid| Attack {
            uuid,
            name: "Phase 2".to_string(),
            power: NotNan::zero(),
            r#type: AttackType::Generic,
            timer: AttackTimer {
                dynamic_timer: Some(timer),
                phase_start: Some(phase_2_start_timer.get().into()),
                dynamic_trigger_cleu_event: Some(CleuEvent {
                    r#type: CleuEventType::SpellAuraApplied,
                    event: 406585,
                    counter: 1,
                }),
                ..Default::default()
            },
        };

        let aoe = move |timer: TimeStep, uuid| Attack {
            uuid,
            name: "AoE".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RaidDamage,
            timer: AttackTimer {
                dynamic_timer: Some(timer),
                phase_start: Some(phase_2_start_timer.get().into()),
                dynamic_trigger_cleu_event: Some(CleuEvent {
                    r#type: CleuEventType::SpellAuraApplied,
                    event: 406585,
                    counter: 1,
                }),
                ..Default::default()
            },
        };

        let meteor_soak = move |timer: TimeStep, uuid| Attack {
            uuid,
            name: "Meteor Soak".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RaidDamage,
            timer: AttackTimer {
                dynamic_timer: Some(timer),
                phase_start: Some(phase_2_start_timer.get().into()),
                dynamic_trigger_cleu_event: Some(CleuEvent {
                    r#type: CleuEventType::SpellAuraApplied,
                    event: 406585,
                    counter: 1,
                }),
                ..Default::default()
            },
        };

        let phase_2 = [
            aoe(
                TimeStep::mm_ss(0, 7),
                AttackUuid::new("d731a8c4-0de0-4ca9-85b3-53208caad0cb"),
            ),
            meteor_soak(
                TimeStep::mm_ss(0, 15),
                AttackUuid::new("515875b2-6118-47a3-99a8-c92f60bf2492"),
            ),
            aoe(
                TimeStep::mm_ss(0, 30),
                AttackUuid::new("f7192bf5-6e84-4a42-b7ec-e9a2a73cbccd"),
            ),
            meteor_soak(
                TimeStep::mm_ss(0, 42),
                AttackUuid::new("43841955-9ee5-4fdc-96aa-e14535f11e42"),
            ),
            aoe(
                TimeStep::mm_ss(0, 49),
                AttackUuid::new("fc9bf288-0df5-47dc-9717-4e45e753f3bf"),
            ),
            aoe(
                TimeStep::mm_ss(1, 1),
                AttackUuid::new("1908edad-2b2e-47fa-bc6f-485b9f9ddd72"),
            ),
            meteor_soak(
                TimeStep::mm_ss(1, 9),
                AttackUuid::new("6284811d-83bb-4dac-991d-4730c8ac6178"),
            ),
        ]
        .into_iter();

        phase_1
            .chain(phase_2)
            .sorted_by_key(|attack| attack.timer.static_timer())
            .collect::<Lookup<Attack>>()
    })
}
