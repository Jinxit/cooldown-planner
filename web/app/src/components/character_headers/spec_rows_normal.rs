use leptos::prelude::*;
use i18n::LocalizedString;
use crate::components::character_headers::spec_row::SpecRow;
use crate::components::icons::chevron::ChevronDirection;

#[component]
pub fn SpecRowsNormal(picking: RwSignal<bool>, character_class: LocalizedString, character_spec: Option<LocalizedString>) -> impl IntoView {
    view! {
        <div
            class="border-2 border-transparent -mt-1"
            on:mousedown=move |ev| {
                if ev.button() != 0 {
                    return;
                }
                picking.set(true);
            }
        >
            <SpecRow chevron_direction=ChevronDirection::Down class=character_class.clone() spec=character_spec.clone() />
        </div>
    }
}
