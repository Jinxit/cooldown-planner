use leptos::prelude::*;
use fight_domain::{AttackTimer, AttackUuid};

#[component]
pub fn AttackTime(uuid: AttackUuid, timer: AttackTimer) -> impl IntoView {
    let column = "attack_timer";
    let row = format!("attack_{}", uuid);
    let attack_timer = timer.static_timer().to_string();

    view! {
        <div
            class="justify-center items-center"
            style:grid-column-start=column
            style:grid-row-start=row
        >
            {attack_timer}
        </div>
    }
}