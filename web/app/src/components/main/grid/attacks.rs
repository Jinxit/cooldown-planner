use crate::{reactive::{ForEach, ForLookup5, ForLookup6}, api::ui_state::UiState};
use fight_domain::{Attack, Lookup};
use leptos::*;

#[component]
pub fn Attacks() -> impl IntoView {
    let ui_state = expect_context::<UiState>();
    view! {
        <ForEach
            each=move || ui_state.attacks()
            bind:attack
        >
            {
                let column = "attack_name";
                let row = format!("attack_{}", attack.uuid);
                view! {
                    <div
                        class="inline-flex my-px min-h-[2rem] justify-center items-center"
                        style:grid-column-start=column
                        style:grid-row-start=row
                    >
                        {attack.name}
                    </div>
                }
            }
        </ForEach>
        <ForEach
            each=move || ui_state.attacks()
            bind:attack
        >
            {
                let column = "attack_timer";
                let row = format!("attack_{}", attack.uuid);
                let attack_timer = attack.timer.static_timer().to_string();
                view! {
                    <div
                        class="inline-flex min-h-[2rem] justify-center items-center"
                        style:grid-column-start=column
                        style:grid-row-start=row
                    >
                        {attack_timer}
                    </div>
                }
            }
        </ForEach>
    }
}
