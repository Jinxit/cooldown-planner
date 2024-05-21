use leptos::prelude::*;

use crate::api::ui_state::UiState;

#[component]
pub fn LockButton() -> impl IntoView {
    let ui_state = use_context::<UiState>().unwrap();

    view! {
        <button
            class="h-12 w-12 transform rounded-md border-2 border-green-950 bg-green-600 text-2xl text-green-950 \
            transition-transform duration-75 \
            hover:bg-green-500 focus-visible:outline focus-visible:outline-1 focus-visible:outline-offset-2 focus-visible:outline-slate-300 active:scale-95"
            on:click=move |_| {
                ui_state.lock_suggestions();
            }
        >

            <div class="fas fa-lock"></div>
        </button>
    }
}
