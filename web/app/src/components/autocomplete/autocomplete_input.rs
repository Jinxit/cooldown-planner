use leptos::html::Input;
use leptos::prelude::*;
use web_sys::KeyboardEvent;
use crate::components::autocomplete::autocomplete_context::use_autocomplete_context;

#[component]
pub fn AutocompleteInput(#[prop(optional)] node_ref: NodeRef<Input>) -> impl IntoView {
    let context = use_autocomplete_context();
    view! {
        <input
            node_ref=node_ref
            node_ref=context.input_node_ref

            on:focus=move |_| {
                context.active.set(true);
            }
            on:blur={
                let context = context.clone();
                move |ev| {
                    context.active.set(false);
                    context.on_blur.as_ref().map(|f| f.call(()));
                }
            }
            on:keydown={
                let context = context.clone();
                move |ev: KeyboardEvent| {
                    match ev.key().as_str() {
                        "Escape" => {
                            context.active.set(false);
                            // delay this by one frame, otherwise this can get recursive which is not supported by wasm-bindgen
                            request_animation_frame(move || {
                                let Some(input_ref) = context.input_node_ref.get_untracked() else { return };
                                let _ = input_ref.blur();
                            });
                        },
                        "ArrowUp" | "ArrowDown" => {
                            ev.prevent_default();
                        },
                        _ => {
                            context.move_keyboard_highlight_to_first_option();
                        }
                    };
                }
            }
        />
    }
}
