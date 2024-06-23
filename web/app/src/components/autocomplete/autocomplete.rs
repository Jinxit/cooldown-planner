use leptos::prelude::*;
use web_sys::KeyboardEvent;
use crate::components::autocomplete::autocomplete_context::provide_autocomplete_context;

#[component]
pub fn Autocomplete(
    #[prop(into, optional)] on_blur: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let context = provide_autocomplete_context(on_blur);

    view! {
        <div
            on:keydown={move |ev: KeyboardEvent| {
                match ev.key().as_str() {
                    "Enter" => {
                        context.select_highlighted();
                        context.active.set(false);
                    }
                    "Tab" => {
                        if context.select_highlighted() {
                            ev.prevent_default();
                            context.active.set(false);
                        }
                    }
                    "ArrowUp" => {
                        context.move_keyboard_highlight(-1);
                    },
                    "ArrowDown" => {
                        context.move_keyboard_highlight(1);
                    },
                    _ => {}
                };
            }}
        >
            {children()}
        </div>
    }
}
