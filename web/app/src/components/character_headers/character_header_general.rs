use leptos::prelude::*;
use planner::PlannerCharacter;
use crate::components::character_headers::name_normal::NameNormal;

#[component]
pub fn CharacterHeaderGeneral(character: PlannerCharacter) -> impl IntoView {
    view! {
        <div class="flex flex-col h-full">
            <NameNormal
                name=character.name.clone()
                realm=character.realm.as_ref().map(|r| r.name.clone())
            />
        </div>
    }
}