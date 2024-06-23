use leptos::prelude::*;
use crate::components::attacks::attack_name::AttackName;
use crate::components::attacks::attack_time::AttackTime;
use crate::context::use_planner;

#[component]
pub fn Attacks() -> impl IntoView {
    let planner = use_planner();
    view! {
        <For
            each={move || planner.get().attacks().iter().map(|a| a.uuid).collect::<Vec<_>>()}
            key=|uuid| *uuid
            children=move |uuid| {
                move || {
                    let planner = planner.read();
                    let attack = planner.attacks().get(&uuid)?.clone();

                    Some(view! {
                        <AttackName uuid name={attack.name.clone()} />
                        <AttackTime uuid timer={attack.timer.clone()} />
                    })
                }
            }
        />
    }
}