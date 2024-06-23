use crate::api::{
    ui_character::{UiCharacter, UiCharacterTemplate},
    ui_state::UiState,
};
use leptos::*;

#[component]
pub fn AddCharacterButton() -> impl IntoView
where
{
    let ui_state = use_context::<UiState>().unwrap();
    view! {
        <div
            class="m-px mb-4 flex items-start"
            style:grid-column-start="add_character"
            style:grid-row-start="character_spells"
        >
            <button
                class="h-8 w-8 transform rounded-md border border-slate-900 bg-slate-500 text-lg \
                text-slate-900 transition-transform duration-75 \
                hover:border-slate-600 hover:bg-slate-400 \
                focus-visible:outline focus-visible:outline-offset-2 focus-visible:outline-slate-300 \
                active:scale-95"
                on:click=move |_| { ui_state.add_ui_character(UiCharacterTemplate::new_unknown()) }
            >
                <div class="fas fa-plus"></div>
            </button>
        </div>
    }
}
