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
use leptos::prelude::*;
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
        |props| view! { <ExperimentsParameters props/> },
        experiments_attacks,
        "boss/experiments.png",
        20,
    )
}

#[derive(Debug, Copy, Clone)]
struct ExperimentsProps {
    neldris_death_timer: RwSignal<Duration>,
    thadrion_active_timer: RwSignal<Duration>,
    thadrion_death_timer: RwSignal<Duration>,
    rionthus_active_timer: RwSignal<Duration>,
}

impl FightProps for ExperimentsProps {
    fn new() -> Self {
        Self {
            neldris_death_timer: create_rw_signal(Duration::mm_ss(2, 0)),
            thadrion_active_timer: create_rw_signal(Duration::mm_ss(1, 8)),
            thadrion_death_timer: create_rw_signal(Duration::mm_ss(3, 20)),
            rionthus_active_timer: create_rw_signal(Duration::mm_ss(2, 15)),
        }
    }
}

#[component]
fn ExperimentsParameters(props: ExperimentsProps) -> impl IntoView {
    view! {
        <div class="flex h-full w-full flex-wrap content-start gap-2 overflow-hidden p-2 transition-all">
            <div>
                <h1 class="block border-b-2 border-r-slate-400 my-auto pb-2">"Neldris"</h1>
                <div class="flex pt-2">
                    <TimerInput
                        label="Death"
                        initial_value=props.neldris_death_timer.get_untracked()
                        set_value=props.neldris_death_timer.write_only()
                    />
                </div>
            </div>

            <div>
                <h1 class="block border-b-2 border-r-slate-400 my-auto pb-2">"Thadrion"</h1>
                <div class="flex pt-2">
                    <TimerInput
                        label="Active"
                        initial_value=props.thadrion_active_timer.get_untracked()
                        set_value=props.thadrion_active_timer.write_only()
                        max_value=props.thadrion_death_timer.read_only()
                    />
                    <div class="ml-2"></div>
                    <TimerInput
                        label="Death"
                        initial_value=props.thadrion_death_timer.get_untracked()
                        set_value=props.thadrion_death_timer.write_only()
                        min_value=props.thadrion_active_timer.read_only()
                    />
                </div>
            </div>

            <div>
                <h1 class="block border-b-2 border-r-slate-400 my-auto pb-2">"Rionthus"</h1>
                <div class="flex pt-2">
                    <TimerInput
                        label="Active"
                        initial_value=props.rionthus_active_timer.get_untracked()
                        set_value=props.rionthus_active_timer.write_only()
                        min_value=props.thadrion_active_timer.read_only()
                    />
                </div>
            </div>
        </div>
    }
}

fn experiments_attacks(props: ExperimentsProps) -> Signal<Lookup<Attack>> {
    let neldris_death_timer = props.neldris_death_timer.memo();
    let thadrion_active_timer = props.thadrion_active_timer.memo();
    let thadrion_death_timer = props.thadrion_death_timer.memo();
    let rionthus_active_timer = props.rionthus_active_timer.memo();
    Signal::derive(move || {
        let on_pull = Attack {
            uuid: AttackUuid::new("d43085ac-670f-4922-a5f8-1ea1d1f598b1"),
            name: "On Pull".to_string(),
            power: NotNan::zero(),
            r#type: AttackType::Generic,
            timer: AttackTimer {
                dynamic_timer: Some(TimeStep::mm_ss(0, 3)),
                ..Default::default()
            },
        };

        let neldris_aoe = move |timer: TimeStep, uuid| Attack {
            uuid,
            name: "Neldris AoE".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RaidDamage,
            timer: AttackTimer {
                dynamic_timer: Some(timer),
                phase_end: Some(neldris_death_timer.get().into()),
                ..Default::default()
            },
        };

        let neldris_dot = move |timer: TimeStep, uuid| Attack {
            uuid,
            name: "Neldris DoT".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RotDamage,
            timer: AttackTimer {
                dynamic_timer: Some(timer),
                phase_end: Some(neldris_death_timer.get().into()),
                ..Default::default()
            },
        };

        let neldris = [
            on_pull,
            neldris_aoe(
                TimeStep::mm_ss(0, 10),
                AttackUuid::new("d7219441-63dd-4f26-883f-753c97deb432"),
            ),
            neldris_dot(
                TimeStep::mm_ss(0, 19),
                AttackUuid::new("cad042d0-375c-409c-9a03-bdbe8f74adb1"),
            ),
            neldris_aoe(
                TimeStep::mm_ss(0, 40),
                AttackUuid::new("9dc58c41-18a7-45e8-8512-717ab7aeb64a"),
            ),
            neldris_dot(
                TimeStep::mm_ss(0, 56),
                AttackUuid::new("a1f2e377-0a2d-4f53-a95a-1ab73f58756f"),
            ),
            neldris_aoe(
                TimeStep::mm_ss(1, 5),
                AttackUuid::new("28db4ed7-1b96-4440-a659-c5d6ee5b8d43"),
            ),
            neldris_dot(
                TimeStep::mm_ss(1, 14),
                AttackUuid::new("3a343809-279b-41a2-84b3-e0ba362c9028"),
            ),
            neldris_aoe(
                TimeStep::mm_ss(1, 35),
                AttackUuid::new("335cf3ff-f9cc-4bb1-9b05-2c62b9993b96"),
            ),
            neldris_dot(
                TimeStep::mm_ss(1, 51),
                AttackUuid::new("f6784738-2ff2-46db-8806-a4db81bf0c2e"),
            ),
            neldris_aoe(
                TimeStep::mm_ss(2, 0),
                AttackUuid::new("dae36a3e-2c7a-4bf8-bb0b-8355c2ce7f0e"),
            ),
            neldris_dot(
                TimeStep::mm_ss(2, 9),
                AttackUuid::new("f27a5274-fa9a-4bc2-a15a-be94a8b09466"),
            ),
            neldris_aoe(
                TimeStep::mm_ss(2, 30),
                AttackUuid::new("97d14315-bccd-4584-acf5-ad8996e9674b"),
            ),
            neldris_dot(
                TimeStep::mm_ss(2, 46),
                AttackUuid::new("0587ea77-ba30-4510-ae53-b041216beecb"),
            ),
            neldris_aoe(
                TimeStep::mm_ss(2, 55),
                AttackUuid::new("de5a8f91-2a69-4bf5-b0b5-02fca2c03c40"),
            ),
            neldris_dot(
                TimeStep::mm_ss(3, 3),
                AttackUuid::new("c4da13bb-b943-49f1-9e2d-1fa0281ecae9"),
            ),
        ]
        .into_iter()
        .filter(|a| a.timer.static_timer() <= neldris_death_timer.get().into());

        let thadrion_debuff = move |timer: TimeStep, uuid| Attack {
            uuid,
            name: "Thadrion Debuff".to_string(),
            power: NotNan::one(),
            r#type: AttackType::Debuffs,
            timer: AttackTimer {
                dynamic_timer: Some(timer - thadrion_active_timer.get().into()),
                phase_start: Some(thadrion_active_timer.get().into()),
                phase_end: Some(thadrion_death_timer.get().into()),
                ..Default::default()
            },
        };

        let thadrion_aoe_8_sec = move |timer: TimeStep, uuid| Attack {
            uuid,
            name: "Thadrion AoE 8 sec".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RaidDamage,
            timer: AttackTimer {
                dynamic_timer: Some(timer - thadrion_active_timer.get().into()),
                phase_start: Some(thadrion_active_timer.get().into()),
                phase_end: Some(thadrion_death_timer.get().into()),
                ..Default::default()
            },
        };

        let thadrion_sequence = move |timer: TimeStep, uuid1, uuid2, uuid3| {
            vec![
                thadrion_debuff(timer, uuid1),
                thadrion_debuff(timer + TimeStep::mm_ss(0, 21), uuid2),
                thadrion_aoe_8_sec(timer + TimeStep::mm_ss(0, 41), uuid3),
            ]
        };

        let thadrion = [
            thadrion_sequence(
                TimeStep::mm_ss(1, 2),
                AttackUuid::new("20fbccbc-5682-48a4-b7a7-514fbb38b4b6"),
                AttackUuid::new("f53e7b00-2a8c-4e86-9c2e-5e57348de062"),
                AttackUuid::new("ad31e027-9110-4d1b-8bea-6c4b65af7cc7"),
            ),
            thadrion_sequence(
                TimeStep::mm_ss(1, 57),
                AttackUuid::new("6002b9c7-56a1-4e64-84d6-826dd1de5d6d"),
                AttackUuid::new("6385246a-68be-4598-ab61-4e576a5a0c63"),
                AttackUuid::new("cb53aad8-e87c-4714-ae43-0ed103aea552"),
            ),
            thadrion_sequence(
                TimeStep::mm_ss(2, 52),
                AttackUuid::new("9c39859b-897e-4b8f-a50c-c901f79ccdce"),
                AttackUuid::new("ea82cec0-394f-4291-9a93-2bceb5185eef"),
                AttackUuid::new("a235add9-c8f6-4825-b999-c57ec28f9420"),
            ),
            thadrion_sequence(
                TimeStep::mm_ss(3, 48),
                AttackUuid::new("76e3c04c-437b-41cd-8149-6f9de775d336"),
                AttackUuid::new("0df341c6-7b25-46b6-ba47-15231e2a61c4"),
                AttackUuid::new("4633b90f-53ae-472c-baee-36d478049bad"),
            ),
        ]
        .into_iter()
        .flatten()
        .filter(|a| {
            a.timer.static_timer() <= thadrion_death_timer.get().into()
                && a.timer.static_timer() >= thadrion_active_timer.get().into()
        });

        let rionthus_beam = move |timer: TimeStep, uuid| Attack {
            uuid,
            name: "Rionthus Beam".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RotDamage,
            timer: AttackTimer {
                dynamic_timer: Some(timer - rionthus_active_timer.get().into()),
                phase_start: Some(rionthus_active_timer.get().into()),
                ..Default::default()
            },
        };

        let rionthus_breath = move |timer: TimeStep, uuid| Attack {
            uuid,
            name: "Rionthus Breath".to_string(),
            power: NotNan::one(),
            r#type: AttackType::Movement,
            timer: AttackTimer {
                dynamic_timer: Some(timer - rionthus_active_timer.get().into()),
                phase_start: Some(rionthus_active_timer.get().into()),
                ..Default::default()
            },
        };

        let rionthus_sequence = move |timer: TimeStep, uuid1, uuid2| {
            vec![
                rionthus_beam(timer, uuid1),
                rionthus_breath(timer + TimeStep::mm_ss(0, 26), uuid2),
            ]
        };

        let rionthus = [
            rionthus_sequence(
                TimeStep::mm_ss(1, 58),
                AttackUuid::new("f28a2d05-4ce9-474f-a384-d1e8f7d47ef9"),
                AttackUuid::new("1fbd25e1-a671-4106-8074-8cef5cbfa0ec"),
            ),
            rionthus_sequence(
                TimeStep::mm_ss(2, 53),
                AttackUuid::new("3f28fec0-2d5d-4134-baef-4eef054fe054"),
                AttackUuid::new("45a94b5d-dfbd-442a-9e23-a4cf2a0a6922"),
            ),
            rionthus_sequence(
                TimeStep::mm_ss(3, 47),
                AttackUuid::new("ef8c5624-422f-455c-8a87-98f7353add3c"),
                AttackUuid::new("2431fa63-b149-4656-b006-e1368c07b691"),
            ),
            rionthus_sequence(
                TimeStep::mm_ss(4, 42),
                AttackUuid::new("a6475ee2-7e50-40f0-ad4b-86e6a0a2366d"),
                AttackUuid::new("5ca55496-13b4-48c0-bc26-6d4f1fb5760e"),
            ),
            rionthus_sequence(
                TimeStep::mm_ss(5, 37),
                AttackUuid::new("13ef5daf-35c5-4fd0-838f-5a48d15d6c76"),
                AttackUuid::new("6e6da0db-2dc2-4290-850a-6524c127fd82"),
            ),
            rionthus_sequence(
                TimeStep::mm_ss(6, 32),
                AttackUuid::new("1e7c4cc4-f45c-4f0d-8fba-efcc2e2f663b"),
                AttackUuid::new("948f2fdf-6446-4b63-bd4a-8e26f57e9a6b"),
            ),
            rionthus_sequence(
                TimeStep::mm_ss(7, 27),
                AttackUuid::new("14708860-f08e-4f70-ae4c-7fc0c2e56773"),
                AttackUuid::new("0a4e5090-1472-45ca-9b2c-9fa9906b705b"),
            ),
        ]
        .into_iter()
        .flatten()
        .filter(|a| a.timer.static_timer() >= rionthus_active_timer.get().into());

        let heal = move |timer: TimeStep,
                         uuid,
                         phase_start: Option<Duration>,
                         phase_end: Option<Duration>| Attack {
            uuid,
            name: "Heal!".to_string(),
            power: NotNan::one(),
            r#type: AttackType::RaidDamage,
            timer: AttackTimer {
                dynamic_timer: Some(timer - phase_start.map(Into::into).unwrap_or_default()),
                phase_start: phase_start.map(Into::into),
                phase_end: phase_end.map(Into::into),
                ..Default::default()
            },
        };

        let dispel = move |timer: TimeStep,
                           uuid,
                           phase_start: Option<Duration>,
                           phase_end: Option<Duration>| Attack {
            uuid,
            name: "Dispel".to_string(),
            power: NotNan::one(),
            r#type: AttackType::Dispels,
            timer: AttackTimer {
                dynamic_timer: Some(timer - phase_start.map(Into::into).unwrap_or_default()),
                phase_start: phase_start.map(Into::into),
                phase_end: phase_end.map(Into::into),
                ..Default::default()
            },
        };

        let first_dispel = {
            let phase_start = TimeStep::from(thadrion_active_timer.get()) + TimeStep::mm_ss(0, 19);
            Attack {
                uuid: AttackUuid::new("57998b42-aeeb-47f2-9642-ee9e67ee572a"),
                name: "Dispel".to_string(),
                power: NotNan::one(),
                r#type: AttackType::Dispels,
                timer: AttackTimer {
                    dynamic_timer: Some(TimeStep::mm_ss(1, 21) - phase_start),
                    phase_start: Some(phase_start),
                    phase_end: Some(thadrion_death_timer.get().into()),
                    ..Default::default()
                },
            }
        };

        let second_dispel = Attack {
            uuid: AttackUuid::new("cdef54a2-29a4-49c9-a993-1b671668a778"),
            name: "Dispel".to_string(),
            power: NotNan::one(),
            r#type: AttackType::Dispels,
            timer: AttackTimer {
                dynamic_timer: Some(TimeStep::mm_ss(1, 51) - thadrion_active_timer.get().into()),
                phase_start: Some(thadrion_active_timer.get().into()),
                phase_end: Some(thadrion_death_timer.get().into()),
                ..Default::default()
            },
        };

        let heal_dispel_sequence = move |timer: TimeStep, thadrion_phase: bool, uuid1, uuid2| {
            vec![
                heal(
                    timer,
                    uuid1,
                    thadrion_phase.then(|| thadrion_active_timer.get()),
                    thadrion_phase.then(|| thadrion_death_timer.get()),
                ),
                dispel(
                    timer + TimeStep::mm_ss(0, 10),
                    uuid2,
                    thadrion_phase.then(|| thadrion_active_timer.get()),
                    thadrion_phase.then(|| thadrion_death_timer.get()),
                ),
            ]
        };

        let dispels = [
            vec![first_dispel, second_dispel].into_iter().collect(),
            heal_dispel_sequence(
                TimeStep::mm_ss(2, 7),
                true,
                AttackUuid::new("d37bbb30-cdbc-4842-9151-ad7e82245641"),
                AttackUuid::new("24c2a50c-16f9-4f58-99be-b5f7a5f4eafb"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(2, 35),
                true,
                AttackUuid::new("43bb4574-7bc1-4b42-a580-2aa030c2a5e9"),
                AttackUuid::new("97eee688-bc27-48e5-87a4-4f60637137b1"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(3, 2),
                false,
                AttackUuid::new("c7140b58-d03d-498a-bc16-d1e3c4c1de2c"),
                AttackUuid::new("ddab2e19-6470-4dd1-98d9-7b807c79fa7d"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(3, 31),
                false,
                AttackUuid::new("7bb29eb1-15b7-4be4-abfb-9d50ca147464"),
                AttackUuid::new("26db457d-5db8-44ee-9c59-9aa2d8d57ed4"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(3, 54),
                false,
                AttackUuid::new("ea004e51-1c11-4dce-afe0-3591c07d91c2"),
                AttackUuid::new("953fffbd-9d02-4f07-97a5-81f37851ad3f"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(4, 22),
                false,
                AttackUuid::new("d3344292-d1f2-4ffb-bae3-a273defc39c8"),
                AttackUuid::new("32ad9da5-84e8-4159-bbff-b4014e2b19dd"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(4, 51),
                false,
                AttackUuid::new("913ba9d2-b897-4ad2-9089-8b68e204c0cf"),
                AttackUuid::new("9dfa4245-29c5-4012-94c7-294123d762de"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(5, 17),
                false,
                AttackUuid::new("95caf4f4-4088-40ee-bf29-0807f2029282"),
                AttackUuid::new("c696776e-5ebb-48d9-9e32-a88aae4cdf09"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(5, 43),
                false,
                AttackUuid::new("beaa23d8-c0cf-419c-9e35-008a24dc4861"),
                AttackUuid::new("7c01c237-6db8-4d45-99ee-af9b8a219c37"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(6, 9),
                false,
                AttackUuid::new("258e21ea-859d-4b60-bc00-1e4efa741040"),
                AttackUuid::new("4a8d477c-9c29-4d64-a55f-eb93441badd5"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(6, 35),
                false,
                AttackUuid::new("ca113299-668f-4e70-b8de-4c57795e62c5"),
                AttackUuid::new("1b1e805c-8506-4236-8b8c-4173ed0ccb66"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(7, 1),
                false,
                AttackUuid::new("f49c6dd4-6ff1-4f37-a7c5-c7ca3e4909ed"),
                AttackUuid::new("4fe3febb-e055-4830-8aec-66a7f9133eff"),
            ),
            heal_dispel_sequence(
                TimeStep::mm_ss(7, 27),
                false,
                AttackUuid::new("79544c8f-6daf-48d4-aed1-c070e5cc7787"),
                AttackUuid::new("a42e41c0-876c-49ba-8da8-097b539a23c9"),
            ),
        ]
        .into_iter()
        .flatten();

        neldris
            .chain(thadrion)
            .chain(rionthus)
            .chain(dispels)
            .filter(|a| a.timer.static_timer() >= a.timer.phase_start.unwrap_or_default())
            .sorted_by_key(|attack| attack.timer.static_timer())
            .collect::<Lookup<Attack>>()
    })
}
