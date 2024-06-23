use leptos::prelude::*;
use fight_domain::CharacterUuid;
use i18n::LocalizedString;
use itertools::Itertools;
use crate::components::character_headers::spec_row::SpecRow;
use crate::components::icons::chevron::ChevronDirection;
use crate::context::{use_class_spec_index, use_planner};

#[component]
pub fn SpecRowsPicking(picking: RwSignal<bool>, character_uuid: CharacterUuid, character_class: Option<LocalizedString>, character_spec: Option<LocalizedString>) -> impl IntoView {
    let planner = use_planner();
    let csi = use_class_spec_index();
    let character_class = character_class.clone();
    let classes_and_specs = if let Some(character_class) = character_class {
        let character_spec = character_spec.clone();
        let specs = csi.specs_for_class(character_class.clone());
        AsyncDerived::new(move || {
            let character_spec = character_spec.clone();
            let character_class = character_class.clone();
            async move {
                specs
                    .await
                    .into_iter()
                    .map(|s| (character_class.clone(), Some(s)))
                    .sorted_by_key(|(_, spec)| spec != &character_spec)
                    .collect::<Vec<_>>()
            }
        })
    } else {
        let classes = csi.classes();
        AsyncDerived::new(move || async move {
            classes
                .await
                .into_iter()
                .map(|c| (c.clone(), None))
                .collect::<Vec<_>>()
        })
    };

    view! {
        <Suspense>
            {move || Suspend(async move {
                let classes_and_specs = classes_and_specs.await;
                view! {
                    <div
                        class="border-2 border-slate-900 -mt-1 w-fit bg-slate-800 rounded-md"
                    >
                        <For
                            each=move || classes_and_specs.clone().into_iter().enumerate().map(|(i, (class, spec))| (i == 0, class, spec))
                            key=|(_, class, spec)| format!("{class:?}-{spec:?}")
                            let:entry
                        >
                            <div class="hover:bg-slate-700 hover:text-white cursor-pointer">
                                <SpecRow
                                    class=entry.1.clone()
                                    spec=entry.2.clone()
                                    chevron_direction={
                                        if entry.0 {
                                            Some(ChevronDirection::Up)
                                        } else {
                                            None
                                        }
                                    }
                                    {..}
                                    on:mousedown=move |ev| {
                                        if ev.button() != 0 {
                                            return;
                                        }
                                        planner.update(|planner| {
                                            if let Some(spec) = &entry.2 {
                                                planner.change_character_spec(character_uuid, spec.clone());
                                                picking.set(false);
                                            } else {
                                                planner.change_character_class(character_uuid, entry.1.clone());
                                            }
                                        })
                                    }
                                />
                            </div>
                        </For>
                    </div>
                }
            })}
        </Suspense>
    }
}
