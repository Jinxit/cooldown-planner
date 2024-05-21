use itertools::Itertools;
use leptos::prelude::*;

use crate::api::ui_state::UiState;
use crate::components::*;
use crate::misc::flatten_ok::FlattenOk;

#[component]
pub fn Planner() -> impl IntoView {
    let tab_open = RwSignal::new(false);

    let ui_state = use_context::<UiState>().unwrap();
    let boss_image = Signal::derive(move || {
        ui_state
            .selected_fight()
            .map(|f| (f.image_path.to_string(), f.image_offset))
            .unwrap_or_default()
    });

    //use_optimizer();

    view! {
        <div
            class="flex h-full min-h-screen w-full select-none flex-col bg-slate-700 text-gray-300 selection:bg-slate-700"
            class=("cursor-progress", move || ui_state.planning())
        >

            <div class="relative flex w-full grow">
                <BossDropdown tab_open/>
            </div>
        </div>
    }
}
