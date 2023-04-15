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
        |props| view! {  <KazzaraParameters props/> },
        kazzara_attacks,
        "boss/kazzara.png",
        40,
    )
}

#[derive(Debug, Copy, Clone)]
struct KazzaraProps {
    eighty_percent_timer: RwSignal<Duration>,
    sixty_percent_timer: RwSignal<Duration>,
    forty_percent_timer: RwSignal<Duration>,
}

impl FightProps for KazzaraProps {
    fn new() -> Self {
        Self {
            eighty_percent_timer: create_rw_signal(Duration::mm_ss(1, 20)),
            sixty_percent_timer: create_rw_signal(Duration::mm_ss(2, 0)),
            forty_percent_timer: create_rw_signal(Duration::mm_ss(3, 0)),
        }
    }
}

#[component]
fn KazzaraParameters(props: KazzaraProps) -> impl IntoView {
    view! {
        <div class="flex h-full w-full flex-wrap content-start gap-2 overflow-hidden p-2 transition-all">
            <TimerInput
                label="80% HP"
                initial_value=props.eighty_percent_timer.get_untracked()
                set_value=props.eighty_percent_timer.write_only()
                max_value=props.sixty_percent_timer.read_only()
            />
            <TimerInput
                label="60% HP"
                initial_value=props.sixty_percent_timer.get_untracked()
                set_value=props.sixty_percent_timer.write_only()
                min_value=props.eighty_percent_timer.read_only()
                max_value=props.forty_percent_timer.read_only()
            />
            <TimerInput
                label="40% HP"
                initial_value=props.forty_percent_timer.get_untracked()
                set_value=props.forty_percent_timer.write_only()
                min_value=props.sixty_percent_timer.read_only()
            />
        </div>
    }
}

fn kazzara_attacks(props: KazzaraProps) -> Signal<Lookup<Attack>> {
    let eighty_percent_timer = props.eighty_percent_timer.memo();
    let sixty_percent_timer = props.sixty_percent_timer.memo();
    let forty_percent_timer = props.forty_percent_timer.memo();
    Signal::derive(move || {
        let on_pull = Attack {
            uuid: AttackUuid::new("755af363-d688-4147-9e30-7bf0f9bf00f9"),
            name: "On Pull".to_string(),
            power: NotNan::zero(),
            r#type: AttackType::Generic,
            timer: AttackTimer {
                dynamic_timer: Some(TimeStep::mm_ss(0, 3)),
                ..Default::default()
            },
        };

        let eighty_percent_aoe = (move || Attack {
            uuid: AttackUuid::new("53d6e795-e1d1-4f79-a250-e2bfe07abbbd"),
            name: "80% HP AoE".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RaidDamage,
            timer: AttackTimer {
                dynamic_timer: Some(Duration::from_secs(3).into()),
                phase_start: Some(eighty_percent_timer.get().into()),
                dynamic_trigger_cleu_event: Some(CleuEvent {
                    r#type: CleuEventType::SpellCastStart,
                    event: 401316,
                    counter: 1,
                }),
                ..Default::default()
            },
        })
        .derive_signal();

        let sixty_percent_aoe = (move || Attack {
            uuid: AttackUuid::new("fdc9c894-985f-4758-81d9-abf2280d5398"),
            name: "60% HP AoE".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RaidDamage,
            timer: AttackTimer {
                dynamic_timer: Some(Duration::from_secs(3).into()),
                phase_start: Some(sixty_percent_timer.get().into()),
                dynamic_trigger_cleu_event: Some(CleuEvent {
                    r#type: CleuEventType::SpellCastStart,
                    event: 401318,
                    counter: 1,
                }),
                ..Default::default()
            },
        })
        .derive_signal();

        let forty_percent_aoe = (move || Attack {
            uuid: AttackUuid::new("f15d1724-2a77-4238-9f73-97849121afd6"),
            name: "40% HP AoE".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RaidDamage,
            timer: AttackTimer {
                dynamic_timer: Some(Duration::from_secs(3).into()),
                phase_start: Some(forty_percent_timer.get().into()),
                dynamic_trigger_cleu_event: Some(CleuEvent {
                    r#type: CleuEventType::SpellCastStart,
                    event: 401319,
                    counter: 1,
                }),
                ..Default::default()
            },
        })
        .derive_signal();

        let knock_aoe = move |uuid, timer: TimeStep| Attack {
            uuid,
            name: "Knock AoE".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RaidDamage,
            timer: AttackTimer {
                dynamic_timer: Some(timer),
                ..Default::default()
            },
        };

        [
            on_pull,
            eighty_percent_aoe(),
            sixty_percent_aoe(),
            forty_percent_aoe(),
            knock_aoe(
                AttackUuid::new("ec9bdd6b-ec2f-499a-9415-0ae043f20465"),
                TimeStep::mm_ss(0, 17),
            ),
            knock_aoe(
                AttackUuid::new("a8f9b2e4-22f5-499f-b157-683514383e04"),
                TimeStep::mm_ss(0, 51),
            ),
            knock_aoe(
                AttackUuid::new("8623ff4f-7669-4922-8ec9-55d1d4849260"),
                TimeStep::mm_ss(1, 34),
            ),
            knock_aoe(
                AttackUuid::new("0c09b21c-355f-424f-abb0-17022cbd0b3f"),
                TimeStep::mm_ss(2, 18),
            ),
            knock_aoe(
                AttackUuid::new("99010925-0e6d-4c5f-9d84-4389b3b2eef5"),
                TimeStep::mm_ss(2, 52),
            ),
            knock_aoe(
                AttackUuid::new("3692b1b9-3869-45e6-90c8-82846f55d987"),
                TimeStep::mm_ss(3, 36),
            ),
            knock_aoe(
                AttackUuid::new("73abc776-e9d5-489c-a638-066a1498725b"),
                TimeStep::mm_ss(4, 10),
            ),
            knock_aoe(
                AttackUuid::new("6fc7f880-9614-498f-839f-3c197adcf192"),
                TimeStep::mm_ss(4, 44),
            ),
            knock_aoe(
                AttackUuid::new("ca447563-ef8d-4ef8-b091-cb6f5f980243"),
                TimeStep::mm_ss(4, 51),
            ),
            knock_aoe(
                AttackUuid::new("7e9c58e8-7b95-4022-9feb-5fc9c0ad97f3"),
                TimeStep::mm_ss(5, 27),
            ),
            knock_aoe(
                AttackUuid::new("b247bd61-ff11-4960-a701-dfe3e5bf52e8"),
                TimeStep::mm_ss(6, 04),
            ),
        ]
        .into_iter()
        .sorted_by_key(|attack| attack.timer.static_timer())
        .collect::<Lookup<Attack>>()
    })
}
