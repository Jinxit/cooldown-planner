use leptos::either::Either;
use leptos::prelude::*;
use i18n::{Locale, LocalizedString};
use crate::components::character_headers::character_header_general::CharacterHeaderGeneral;
use crate::components::character_headers::character_header_player::CharacterHeaderPlayer;
use crate::context::use_planner;

#[component]
pub fn CharacterHeaders() -> impl IntoView {
    let planner = use_planner();
    view! {
        <div class="contents">
        <For
            each={move || planner.get().characters().iter().map(|c| c.uuid).collect::<Vec<_>>()}
            key=|uuid| *uuid
            children=move |uuid| {
                let character = Memo::new(move |_| planner.read().characters().get(&uuid).cloned());
                move || {
                    let character = character.get()?;

                    let contents = if character.is_general() {
                        Either::Left(view! {
                            <CharacterHeaderGeneral character />
                        })
                    } else {
                        Either::Right(view! {
                            <CharacterHeaderPlayer character />
                        })
                    };

                    Some(view! {
                        <div
                            class="relative h-14 w-full mt-4 sm:mt-0 text-slate-300"
                            style:grid-row-start="character_name"
                            style:grid-column-start=format!("character_{}", uuid)
                        >
                            {contents}
                        </div>
                    })
                }
            }
        />
        </div>
    }
}