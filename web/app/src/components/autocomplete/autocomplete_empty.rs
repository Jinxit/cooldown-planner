use leptos::prelude::*;
use crate::components::autocomplete::autocomplete_context::use_autocomplete_context;

#[component]
pub fn AutocompleteEmpty(children: Children) -> impl IntoView {
    let context = use_autocomplete_context();

    view! {
        <div style:display={move || if context.should_show_empty.get() { "inherit" } else { "none" }}>
            {children()}
        </div>
    }
}
