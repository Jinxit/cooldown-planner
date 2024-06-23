use leptos::prelude::*;

#[component]
pub fn XMark() -> impl IntoView {
    view! {
        <button class="w-5">
            <div class="fa-solid fa-xmark text-sm"></div>
        </button>
    }
}