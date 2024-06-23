use leptos::prelude::*;
use planner::PlannerCharacter;
use crate::components::character_headers::class_color_bar::ClassColorBar;
use crate::components::character_headers::name_normal::NameNormal;
use crate::components::character_headers::spec_rows::SpecRows;
use crate::components::icons::x_mark::XMark;
use crate::context::use_planner;

#[component]
pub fn CharacterHeaderPlayerNormal(editing: RwSignal<bool>, character: PlannerCharacter) -> impl IntoView {
    let planner = use_planner();
    let character_uuid = character.uuid;
    view! {
        <div class="w-fit flex flex-row items-start border-l-2 border-transparent cursor-text">
            <div class="hover:text-white">
                <XMark {..}
                    on:mousedown=move |ev| {
                        if ev.button() != 0 {
                            return;
                        }
                        planner.update(|planner| {
                            planner.remove_character(character_uuid);
                        })
                    }
                />
            </div>
            <ClassColorBar class=character.class.clone() />
            <div
                class="hover:text-white"
                on:mousedown=move |ev| {
                    if ev.button() != 0 {
                        return;
                    }
                    editing.set(true)
                }
            >
                <NameNormal
                    name=character.name.clone()
                    realm=character.realm.as_ref().map(|r| r.name.clone())
                />
            </div>
        </div>
        <SpecRows character />
    }
}
