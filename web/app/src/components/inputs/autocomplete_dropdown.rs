use crate::misc::hash_ext::Hashable;
use leptos::html::{Div, Input};
use leptos::*;
use std::cmp::{max, min};
use std::hash::Hash;
use wasm_bindgen::JsCast;
use web_sys::{console, FocusEvent, KeyboardEvent};

fn use_loaded<F>(f: F)
where
    F: Fn() + Clone + 'static,
{
    create_effect(move |_| {
        let suspense = use_context::<SuspenseContext>();
        if suspense.is_none() || suspense.unwrap().ready() {
            let f = f.clone();
            request_animation_frame(move || {
                create_effect(move |_| {
                    f();
                })
            });
        }
    });
}

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
    T: Clone + Eq + 'static + std::fmt::Debug,
    F: FnOnce(Signal<String>) -> Signal<Vec<T>> + 'static,
    KF: Fn(&T) -> K + Copy + 'static,
    K: Eq + Hash + 'static,
    G: Fn(&T) + 'static,
    G2: Fn(&str) + 'static,
    H: Fn() + 'static,
    V: Fn(T, &str, Signal<bool>) -> IV + Copy + 'static,
    V2: Fn(&str, Signal<bool>) -> IV2 + Copy + 'static,
    IV: IntoView,
    IV2: IntoView,
{
    let on_select = store_value(on_select);
    let on_select_custom = store_value(on_select_custom);
    let on_blur = store_value(on_blur);
    let (query, set_query) = create_signal(String::new());
    let (selection, set_selection) = create_signal(0_usize);
    let is_selected = store_value(create_selector(selection));
    let input_ref: NodeRef<Input> = create_node_ref();
    let outer_ref: NodeRef<Div> = create_node_ref();

    // TODO: check that this still works
    use_loaded(move || {
        if let Some(input) = input_ref.get() {
            input.focus().unwrap();
        }
    });

    let entries = autocomplete(query.into());
    let num_entries = create_memo(move |_| {
        let e = entries();
        e.len()
    });

    view! {
        <div class="w-full overflow-y-visible px-2" node_ref=outer_ref>
            <input
                type="text"
                id="timer-80"
                class="w-full z-40 relative overflow-visible rounded-md border-2 border-slate-500 bg-slate-900 px-1 text-slate-300 focus-visible:outline-none"
                node_ref=input_ref
                prop:value=query
                on:input=move |ev| {
                    set_query(event_target_value(&ev).replace(' ', ""));
                    set_selection(0);
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
                            set_selection(0);
                            set_query(String::new());
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
                            let count = num_entries();
                            set_selection
                                .update(|s| {
                                    *s = min(*s + 1, count);
                                });
                        }
                        "Enter" => {
                            ev.prevent_default();
                            let s = selection.get_untracked();
                            let entries = entries();
                            if let Some(entry) = entries.get(s) {
                                on_select.with_value(|on_select| on_select(entry));
                            } else {
                                on_select_custom.with_value(|on_select_custom| on_select_custom(&query()));
                            }
                        }
                        _ => {}
                    };
                }
            />
            <div class="-mt-1 relative z-30 flex w-full cursor-pointer flex-col divide-y divide-slate-950 rounded-b-md border-2 border-t-0 border-slate-950 bg-slate-800 pt-1 group">
                <Suspense fallback=|| ()>
                    {move || entries().into_iter().enumerate().map(|(index, entry)| {
                        view! {
                            <Suspense fallback=|| ()>
                                <div
                                    tabindex=-1
                                    on:click=move |ev| {
                                        let entries = entries();
                                        if let Some(entry) = entries.get(index) {
                                            on_select.with_value(|on_select| on_select(entry));
                                        }
                                    }
                                >
                                    {view(
                                        entry.clone(),
                                        &query(),
                                        Signal::derive(move || is_selected.get_value().selected(index)),
                                    )}
                                </div>
                            </Suspense>
                        }
                    }).collect_view()}
                    <Show when=move || !query().is_empty() fallback=|| ()>
                        <div
                            tabindex=-1
                            on:click=move |ev| {
                                on_select_custom.with_value(|on_select_custom| on_select_custom(&query()));
                            }
                        >
                            {move || view_custom(

                                &query(),
                                Signal::derive(move || is_selected.get_value().selected(num_entries())),
                            )}
                        </div>
                    </Show>
                </Suspense>
            </div>
        </div>
    }
}
