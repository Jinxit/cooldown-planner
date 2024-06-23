use leptos::prelude::*;
use crate::api::write_to_clipboard;
use crate::context::use_planner;

#[component]
pub fn CopyButton() -> impl IntoView {
    let planner = use_planner();
    view! {
        <button class="h-12 w-12 transform rounded-md border-2 border-green-950 bg-green-600 text-2xl text-green-950 \
        transition-transform duration-75 \
        hover:bg-green-500 focus-visible:outline focus-visible:outline-1 focus-visible:outline-offset-2 focus-visible:outline-slate-300 active:scale-95"
            on:mousedown=move |ev| {
                if ev.button() != 0 {
                    return;
                }
                if let Some(exported) = planner.get().export() {
                    write_to_clipboard(exported, || {});
                } else {
                    error!("Failed to export plan");
                }
            }
        >
            <div class="fas fa-copy"></div>
        </button>
    }
}
