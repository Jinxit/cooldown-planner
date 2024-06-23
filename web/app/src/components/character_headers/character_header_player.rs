use leptos::either::Either;
use leptos::prelude::*;
use planner::PlannerCharacter;
use crate::components::character_headers::character_header_player_editing::CharacterHeaderPlayerEditing;
use crate::components::character_headers::character_header_player_normal::CharacterHeaderPlayerNormal;

#[component]
pub fn CharacterHeaderPlayer(character: PlannerCharacter) -> impl IntoView {
    // if there is no name, instantly go into edit mode
    let editing = RwSignal::new(character.name.is_none());
    view! {
        <div class="flex flex-col h-full">
            {move || if editing.get() { Either::Left(view! {
                <CharacterHeaderPlayerEditing current_character_uuid=character.uuid editing />
            })} else { Either::Right(view! {
                <CharacterHeaderPlayerNormal editing character=character.clone() />
            })}}
        </div>
    }
}
