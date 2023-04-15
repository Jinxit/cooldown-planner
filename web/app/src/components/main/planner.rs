use std::collections::HashMap;
use std::sync::Arc;

use crate::api::ui_assignment::UiAssignmentState;
use crate::api::ui_character::{UiCharacter, UiCharacterTemplate};
use crate::api::ui_fight::UiFight;
use crate::api::ui_state::UiState;
use crate::api::use_optimizer;
use crate::components::*;
use crate::localization::{encounter, preferences};
use crate::misc::flatten_ok::FlattenOk;
use crate::misc::localized_string_with_context::LocalizedStringWithContext;
use crate::reactive::blank_suspense::BlankSuspense;
use crate::reactive::map::Map;
use auto_battle_net::LocalizedString;
use fight_domain::{
    Attack, AttackUuid, Character, CharacterUuid, Lookup, LookupKey, Spell, SpellUuid,
};
use itertools::Itertools;
use leptos::*;
use optimizer::Assignment;

#[component]
pub fn Planner() -> impl IntoView {
    let tab_open = create_rw_signal(false);

    let ui_state = expect_context::<UiState>();
    let boss_image = Signal::derive(move || {
        ui_state
            .selected_fight()
            .map(|f| (f.image_path.to_string(), f.image_offset))
            .unwrap_or_default()
    });

    //use_optimizer();

    view! {
        <div
            class="flex h-full min-h-screen w-full select-none flex-col bg-slate-700 text-gray-300 selection:bg-slate-700"
            class=("cursor-progress", move || ui_state.planning())
        >
            <BlankSuspense>
                <Nav tab_open>
                    <Tab slot>
                        <TabHeader slot>
                            <div class="fa-solid fa-dragon mr-1"></div>
                            <span>{move || encounter().localize()}</span>
                        </TabHeader>
                        <TabBody slot>
                            <BlankSuspense>
                                <NavTabBodyBackground image=boss_image />
                                {move || ui_state.selected_fight().map(|f| f.parameters.clone())}
                            </BlankSuspense>
                        </TabBody>
                    </Tab>
                    <Tab slot>
                        <TabHeader slot>
                            <div class="fa-solid fa-gear mr-1"></div>
                            <span>{move || preferences().localize()}</span>
                        </TabHeader>
                        <TabBody slot>
                            <p>"empty"</p>
                        </TabBody>
                    </Tab>
                </Nav>
            </BlankSuspense>
            <div class="relative flex w-full grow">
                <BlankSuspense>
                    <BossDropdown tab_open />
                </BlankSuspense>
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
