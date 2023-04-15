use crate::api::ui_fight::UiFight;
use crate::api::ui_state::UiState;
use crate::misc::localized_string_with_context::LocalizedStringWithContext;
use leptos::*;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use std::thread::ThreadId;

#[component]
pub fn BossDropdown(#[prop(into)] tab_open: Signal<bool>) -> impl IntoView {
    let ui_state = expect_context::<UiState>();
    let (picking, set_picking) = create_signal(false);
    view! {
        <div
            class="absolute right-0 z-50 rounded-l-md border-r-0 border-slate-900 transition-all"
            class=("border-2", picking)
            class=("bg-slate-800", picking)
            class=("border-transparent", move || !picking())
            class=("-top-2", move || !tab_open())
            class=("-top-11", tab_open)
        >
            <div class="flex w-full flex-col">
                <div class="flex">
                    <div class="flex-grow"></div>
                    <button class="flex items-center justify-end space-x-2 px-2 pt-2 hover:text-white focus-visible:outline focus-visible:outline-1 focus-visible:outline-offset-2 focus-visible:outline-slate-300"
                        on:click=move |_| {
                            set_picking.update(|picking| {
                                *picking = !*picking;
                            })
                        }
                    >
                        <h2 class="text-right font-title text-2xl text-shadow-outline"
                            class=("shadow-black", move || tab_open() &&!picking())
                        >
                            {move || ui_state.selected_fight().map(|f| f.encounter_name.localize())}
                        </h2>
                        <div>
                            <Show when=picking
                                fallback=move || view! {
                                    <div class="fas fa-chevron-up"></div>
                                }
                            >
                                <div class="fas fa-chevron-down"></div>
                            </Show>
                        </div>
                    </button>
                </div>
                <Show when=picking fallback=move || ()>
                    <div class="flex flex-col">
                        <For
                            each=move || ui_state.fights().clone().into_iter().enumerate()
                            key=|(_, fight)| fight.encounter_id
                            view=move | (index, fight)| {
                                view! {
                                    <div class="flex">
                                        <div class="flex-grow"></div>
                                        <Show when=move || index != ui_state.selected_fight_index() fallback=|| ()>
                                            <a
                                                class="px-2 font-title text-lg cursor-pointer hover:text-white"
                                                on:click=move |_| {
                                                    ui_state.set_selected_fight_index(index);
                                                    set_picking(false);
                                                }
                                            >
                                                {move || ui_state.fights().get(index).map(|f| f.encounter_name.localize())}
                                            </a>
                                        </Show>
                                        <Show when=move || index == ui_state.selected_fight_index() fallback=|| ()>
                                            <a
                                                class="px-2 font-title text-lg text-slate-400"
                                            >
                                                {move || ui_state.fights().get(index).map(|f| f.encounter_name.localize())}
                                            </a>
                                        </Show>
                                    </div>
                                }
                            }
                        />
                    </div>
                </Show>
            </div>
        </div>
    }
}
