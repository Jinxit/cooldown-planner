use leptos::prelude::*;
use crate::components::main::main_grid::MainGrid;

#[component]
pub fn Main(tab_open: RwSignal<bool>, children: Children) -> impl IntoView {
    view! {
        <main class="relative flex w-full grow font-sans font-medium text-slate-100 transition-[padding]">
            <MainGrid tab_open>
                {children()}
            </MainGrid>
        </main>
    }
}