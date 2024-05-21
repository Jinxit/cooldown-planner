use crate::misc::hash_ext::Hashable;
use leptos::html::{Div, Input};
use leptos::prelude::*;
use leptos::prelude::wrappers::read::Signal;
use std::cmp::{max, min};
use std::hash::Hash;
use wasm_bindgen::JsCast;
use web_sys::{console, FocusEvent, KeyboardEvent};
use std::sync::Arc;

/*
fn use_loaded<F>(f: F)
where
    F: Fn() + Clone + 'static,
{
    Effect::new(move |_| {
        let suspense = use_context::<SuspenseContext>();
        if suspense.is_none() || suspense.unwrap().ready() {
            let f = f.clone();
            request_animation_frame(move || {
                Effect::new(move |_| {
                    f();
                })
            });
        }
    });
}
 */

#[component]
pub fn AutocompleteDropdown<T, F, KF, K, G, G2, H, V, V2, IV, IV2>(
    autocomplete: F,
    key: KF,
    on_select: G,
    on_select_custom: G2,
    on_blur: H,
    view: V,
    view_custom: V2,
) -> impl IntoView
where
    T: Clone + Eq + 'static + std::fmt::Debug + Send + Sync,
    F: FnOnce(Signal<String>) -> Signal<Vec<T>> + 'static,
    KF: Fn(&T) -> K + Copy + 'static,
    K: Eq + Hash + 'static,
    G: Fn(&T) + Send + Sync + 'static,
    G2: Fn(&str) + Send + Sync + 'static,
    H: Fn() + Send + Sync + 'static,
    V: Fn(T, &str, Signal<bool>) -> IV + Copy + Send + Sync + 'static,
    V2: Fn(&str, Signal<bool>) -> IV2 + Copy + Send + Sync + 'static,
    IV: IntoView + 'static,
    IV2: IntoView + 'static,
{
    let on_select = StoredValue::new(on_select);
    let on_select_custom = StoredValue::new(on_select_custom);
    let on_blur = StoredValue::new(on_blur);
    let (query, set_query) = signal(String::new());
    let (selection, set_selection) = signal(0_usize);
    let is_selected = StoredValue::new(selector::Selector::new(move || selection.get()));
    let input_ref: NodeRef<Input> = NodeRef::new();
    let outer_ref: NodeRef<Div> = NodeRef::new();

    // TODO: check that this still works
    // TODO 2: switch this to use RenderEffect and see if that works
    /*use_loaded(move || {
        if let Some(input) = input_ref.get() {
            input.focus().unwrap();
        }
    });*/

    let entries = autocomplete(query.into());
    let num_entries = Memo::new(move |_| {
        let e = entries.get();
        e.len()
    });

    view! {
        <div class="w-full overflow-y-visible px-2" node_ref=outer_ref>
            <input
                type="text"
                id="timer-80"
                class="w-full z-40 relative overflow-visible rounded-md border-2 border-slate-500 bg-slate-900 px-1 text-slate-300 focus-visible:outline-none"
                node_ref=input_ref
                prop:value=move || query().clone()
                on:input=move |ev| {
                    set_query.set(event_target_value(&ev).replace(' ', ""));
                    set_selection.set(0_usize);
                }

                on:focusout=move |ev: FocusEvent| {
                    if let Some((input_ref, outer_ref)) = input_ref.get().zip(outer_ref.get()) {
                        let receiver = ev
                            .related_target()
                            .map(|r| r.unchecked_into::<web_sys::Node>());
                        if !outer_ref.contains(receiver.as_ref()) {
                            on_blur.with_value(|on_blur| on_blur());
                        }
                    }
                }

                on:keydown=move |ev: KeyboardEvent| {
                    match ev.key().as_ref() {
                        "Escape" => {
                            ev.prevent_default();
                            set_selection.set(0_usize);
                            set_query.set(String::new());
                            on_blur.with_value(|on_blur| on_blur());
                        }
                        "ArrowUp" => {
                            ev.prevent_default();
                            set_selection
                                .update(|s| {
                                    *s = max(1, *s) - 1;
                                });
                        }
                        "ArrowDown" => {
                            ev.prevent_default();
                            let count = num_entries.get();
                            set_selection
                                .update(|s| {
                                    *s = min(*s + 1, count);
                                });
                        }
                        "Enter" => {
                            ev.prevent_default();
                            let s = selection.get_untracked();
                            let entries = entries.get();
                            if let Some(entry) = entries.get(s) {
                                on_select.with_value(|on_select| on_select(entry));
                            } else {
                                on_select_custom
                                    .with_value(|on_select_custom| on_select_custom(&query.get()));
                            }
                        }
                        _ => {}
                    };
                }
            />

            <div class="-mt-1 relative z-30 flex w-full cursor-pointer flex-col divide-y divide-slate-950 rounded-b-md border-2 border-t-0 border-slate-950 bg-slate-800 pt-1 group">
                <Suspense fallback=|| ()>
                    {move || {
                        entries
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(index, entry)| {
                                view! {
                                    <Suspense fallback=|| ()>
                                        <div
                                            tabindex=-1
                                            on:click=move |ev| {
                                                let entries = entries.get();
                                                if let Some(entry) = entries.get(index) {
                                                    on_select.with_value(|on_select| on_select(entry));
                                                }
                                            }
                                        >

                                            {view(
                                                entry.clone(),
                                                &query.get(),
                                                Signal::derive(move || {
                                                    is_selected.get_value().selected(index)
                                                }),
                                            )}

                                        </div>
                                    </Suspense>
                                }
                            })
                            .collect::<Vec<_>>()
                    }} <Show when=move || !query.get().is_empty() fallback=|| ()>
                        <div
                            tabindex=-1
                            on:click=move |ev| {
                                on_select_custom
                                    .with_value(|on_select_custom| on_select_custom(&query.get()));
                            }
                        >

                            {move || view_custom(
                                &query.get(),
                                Signal::derive(move || {
                                    is_selected.get_value().selected(num_entries.get())
                                }),
                            )}

                        </div>
                    </Show>
                </Suspense>
            </div>
        </div>
    }
}
