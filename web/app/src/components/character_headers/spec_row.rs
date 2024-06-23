use leptos::prelude::*;
use i18n::LocalizedString;
use crate::components::character_headers::class_color_bar::ClassColorBar;
use crate::components::character_headers::spec_name::SpecName;
use crate::components::icons::chevron::{Chevron, ChevronDirection};

// rounded-t-md border-l-2 border-t-2 border-r-2 border-slate-900 bg-slate-800
#[component]
pub fn SpecRow(#[prop(into)] chevron_direction: Option<ChevronDirection>, class: LocalizedString, spec: Option<LocalizedString>) -> impl IntoView {
    view! {
        <div class="w-fit flex flex-row items-center hover:text-white">
            <div class=("invisible", chevron_direction.is_none())>
                <Chevron direction=chevron_direction.unwrap_or(ChevronDirection::Down) />
            </div>
            <ClassColorBar class=Some(class.clone()) />
            <SpecName class spec />
        </div>
    }
}
