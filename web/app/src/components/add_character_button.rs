use leptos::prelude::*;
use planner::PlannerCharacterTemplate;

use crate::context::use_planner;

#[component]
pub fn AddCharacterButton() -> impl IntoView
where
{
    let planner = use_planner();
    view! {
        <div
            class="m-px mb-4 flex items-start justify-end"
            style:grid-column-start="add_character"
            style:grid-row-start="character_spells"
        >
            <button
                class="h-8 w-8 transform rounded-md border border-slate-900 bg-slate-500 text-lg \
                text-slate-900 transition-transform duration-75 \
                hover:border-slate-600 hover:bg-slate-400 \
                focus-visible:outline focus-visible:outline-offset-2 focus-visible:outline-slate-300 \
                active:scale-95"
                on:mousedown=move |ev| {
                    if ev.button() != 0 {
                        return;
                    }
                    planner.update(|planner| {
                        planner.add_character(PlannerCharacterTemplate::Unknown);
                    })
                }
            >
                <div class="fas fa-plus"></div>
            </button>
        </div>
    }
}
