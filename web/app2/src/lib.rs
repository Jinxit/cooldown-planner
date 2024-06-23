#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::zero_prefixed_literal)]
#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use components::*;

mod api;
mod components;
mod context;
mod misc;
mod reactive;
pub mod serverfns;
pub mod session;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html class="bg-slate-700 text-gray-300 selection:bg-slate-700"/>
        <Stylesheet id="leptos" href="/pkg/cooldown-planner.css"/>
        <Title text="Cargo Leptos"/>
        <Router>
            <Routes>
                <Route path="" view=InnerApp/>
            </Routes>
        </Router>
    }
}

#[component]
fn InnerApp() -> impl IntoView {
    context::provide();
    view! {
        <LoggedIn fallback=move || {
            view! { <LoginButton /> }
        }>
            <Planner/>
        </LoggedIn>
    }
}
