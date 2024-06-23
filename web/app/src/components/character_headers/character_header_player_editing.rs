use leptos::prelude::*;
use fight_domain::CharacterUuid;
use crate::components::character_headers::name_editing::NameEditing;
use crate::components::icons::x_mark::XMark;
use crate::context::use_planner;

#[component]
pub fn CharacterHeaderPlayerEditing(current_character_uuid: CharacterUuid, editing: RwSignal<bool>) -> impl IntoView {
    let planner = use_planner();
    view! {
        <div class="flex flex-row items-start">
            <div class="hover:text-white">
                <XMark {..}
                    on:mousedown=move |ev| {
                        if ev.button() != 0 {
                            return;
                        }
                        planner.update(|planner| {
                            planner.remove_character(current_character_uuid);
                        })
                    }
                />
            </div>
            <NameEditing current_character_uuid editing />
        </div>
    }
}
