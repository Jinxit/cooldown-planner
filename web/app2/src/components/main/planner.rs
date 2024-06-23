use itertools::Itertools;
use leptos::*;
use i18n::{encounter, preferences};

use crate::api::ui_state::UiState;
use crate::api::use_optimizer;
use crate::components::*;
use crate::misc::flatten_ok::FlattenOk;
use crate::misc::localized_string_with_context::LocalizedStringWithContext;

#[component]
pub fn Planner() -> impl IntoView {
    let tab_open = RwSignal::new(false);

    let ui_state = use_context::<UiState>().unwrap();
    let boss_image = Signal::derive(move || {
        ui_state
            .selected_fight()
            .map(|f| (f.image_path.to_string(), f.image_offset))
            .unwrap_or_default()
    });

    use_optimizer();

    view! {
        <div
            class="flex h-full min-h-screen w-full select-none flex-col bg-slate-700 text-gray-300 selection:bg-slate-700"
            class=("cursor-progress", move || ui_state.planning())
        >
            <Suspense>
                <Nav tab_open>
                    <Tab slot>
                        <TabHeader slot>
                            <div class="fa-solid fa-dragon mr-1"></div>
                            <span>{move || encounter().localize().to_string()}</span>
                        </TabHeader>
                        <TabBody slot>
                            <Suspense>
                                <NavTabBodyBackground image=boss_image />
                                {move || ui_state.selected_fight().map(|f| f.parameters.run())}
                            </Suspense>
                        </TabBody>
                    </Tab>
                    <Tab slot>
                        <TabHeader slot>
                            <div class="fa-solid fa-gear mr-1"></div>
                            <span>{move || preferences().localize().to_string()}</span>
                        </TabHeader>
                        <TabBody slot>
                            <p>"empty"</p>
                        </TabBody>
                    </Tab>
                </Nav>
            </Suspense>

            <div class="relative flex w-full grow">
                <Suspense>
                    <BossDropdown tab_open />
                </Suspense>
                <GridSkeleton tab_open>
                    <CornerButtons />
                    <CharacterBackgrounds tab_open />
                    <CharacterHeaders />
                    <AddCharacterButton />
                    <CharacterSpells />
                    <Attacks />
                    <Assignments />
                </GridSkeleton>
            </div>
        </div>
    }
}
