use leptos::prelude::*;
use web_sys::KeyboardEvent;
use crate::components::autocomplete::autocomplete_context::use_autocomplete_context;

#[component]
pub fn AutocompleteItems(children: Children) -> impl IntoView {
    let context = use_autocomplete_context();
    view! {
        <div
            style:display={move || if context.active.get() { "inherit" } else { "none" }}
            node_ref=context.items_node_ref
            on:mouseleave={
                let context = context.clone();
                move |_| {
                    context.set_keyboard_highlight_to_mouse_highlight();
                }
            }
            on:keydown={
                let context = context.clone();
                move |ev: KeyboardEvent| {
                    match ev.key().as_str() {
                        "Enter" => {
                            context.active.set(false);
                            context.select_highlighted();
                        }
                        "ArrowUp" => {
                            context.move_keyboard_highlight(-1);
                        },
                        "ArrowDown" => {
                            context.move_keyboard_highlight(1);
                        },
                        _ => {}
                    };
                }
            }
        >
            {children()}
        </div>
    }
}
