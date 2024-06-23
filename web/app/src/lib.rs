#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::zero_prefixed_literal)]
#![allow(incomplete_features)]
#![allow(unstable_name_collisions)]
#![feature(stmt_expr_attributes)]
#![feature(impl_trait_in_assoc_type)]
#![feature(is_none_or)]

use std::hash::Hash;

use convert_case::Casing;
use itertools::Itertools;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    params::Params,
    StaticSegment,
};
use rand::Rng;
use wasm_bindgen::JsCast;

use i18n::{encounter, LocalizedString, preferences};
use planner::{PlannerCharacterTemplate, PlannerFight, PlannerRealm};

#[cfg(feature = "render")]
use crate::api::use_optimizer;
#[cfg(feature = "render")]
use crate::components::add_character_button::AddCharacterButton;
#[cfg(feature = "render")]
use crate::components::assignment_icons::assignment_icons::AssignmentIcons;
#[cfg(feature = "render")]
use crate::components::attacks::attacks::Attacks;
#[cfg(feature = "render")]
use crate::components::character_backgrounds::character_backgrounds::CharacterBackgrounds;
#[cfg(feature = "render")]
use crate::components::character_headers::character_headers::CharacterHeaders;
#[cfg(feature = "render")]
use crate::components::character_spell_toggles::character_spell_toggles::CharacterSpellToggles;
#[cfg(feature = "render")]
use crate::components::corner_buttons::CornerButtons;
#[cfg(feature = "render")]
use crate::components::login::logged_in::LoggedIn;
#[cfg(feature = "render")]
use crate::components::main::main::Main;
#[cfg(feature = "render")]
use crate::components::nav::{Nav, NavTabBodyBackground, Tab, TabBody, TabHeader};
#[cfg(feature = "render")]
use crate::context::{use_class_spec_index, use_planner, UserContext, with_workers};
#[cfg(feature = "render")]
use crate::misc::localized_string_with_context::LocalizedStringWithContext;

#[cfg(feature = "render")]
pub mod api;
#[cfg(feature = "render")]
pub mod components;
#[cfg(feature = "render")]
pub mod context;
#[cfg(feature = "render")]
mod misc;
#[cfg(feature = "render")]
mod reactive;
pub mod serverfns;
pub mod session;

#[cfg(feature = "render")]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();

    view! {
        <Html class="bg-slate-700 text-gray-300 selection:bg-slate-700"/>
        <Stylesheet id="leptos" href="/pkg/cooldown-planner.css"/>
        <Title text="Cargo Leptos"/>
        <Router>
            <Routes fallback>
                <Route path=StaticSegment("") view=InnerApp/>
            </Routes>
        </Router>
    }
}

#[cfg(feature = "render")]
#[component]
pub fn InnerApp() -> impl IntoView {
    context::provide();
    let planner = use_planner();

    view! {
            //<ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors/> }>
                //<LoggedIn>
                    <Planner/>
                //</LoggedIn>
            //</ErrorBoundary>
    }
}

#[cfg(feature = "render")]
#[component]
pub fn Planner() -> impl IntoView {
    let tab_open = RwSignal::new(false);
    use_optimizer();
    view! {
        <div class="flex h-full min-h-screen w-full select-none flex-col">
            <Header tab_open />
            <Main tab_open>
                <CharacterBackgrounds tab_open />
                <CornerButtons/>
                <AddCharacterButton/>
                <Attacks/>
                <AssignmentIcons/>
                <CharacterSpellToggles/>
                <CharacterHeaders/>
            </Main>
        </div>
    }
}

#[cfg(feature = "render")]
#[component]
pub fn Header(tab_open: RwSignal<bool>) -> impl IntoView {
    let planner = use_planner();
    let boss_image = Signal::derive(move || {
        planner
            .get()
            .selected_fight()
            .and_then(|f| f.data().clone())
            .map(|f| (f.image_path.to_string(), f.image_offset))
            .unwrap_or_default()
    });
    let user = use_context::<UserContext>().unwrap();

    view! {
        <header>
            <Nav tab_open>
                <Tab slot>
                    <TabHeader slot>
                        <div class="fa-solid fa-dragon mr-1"></div>
                        <span>{encounter().localize(user)}</span>
                    </TabHeader>
                    <TabBody slot>
                        <NavTabBodyBackground image=boss_image />
                        //{move || planner.get().selected_fight().map(|f| f.parameters.run())}
                    </TabBody>
                </Tab>
                <Tab slot>
                    <TabHeader slot>
                        <div class="fa-solid fa-gear mr-1"></div>
                        <span>{preferences().localize(user)}</span>
                    </TabHeader>
                    <TabBody slot>
                        <p>"empty"</p>
                    </TabBody>
                </Tab>
            </Nav>
        </header>
    }
}
