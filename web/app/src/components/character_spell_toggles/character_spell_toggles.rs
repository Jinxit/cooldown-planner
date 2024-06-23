use leptos::prelude::*;
use crate::components::character_spell_toggles::character_spell_toggle::CharacterSpellToggle;
use crate::context::use_planner;

#[component]
pub fn CharacterSpellToggles() -> impl IntoView {
    let planner = use_planner();
    view! {
        <For
            each={move || planner.get().characters().iter().map(|c| c.uuid).collect::<Vec<_>>()}
            key=|character_uuid| *character_uuid
            children=move |character_uuid| {
                let column = move || format!("character_{}", character_uuid);
                let row = "character_spells";
                view! {
                    <div
                        class="flex justify-center items-center flex-wrap"
                        style:grid-column-start=column
                        style:grid-row-start=row
                    >
                        <For
                            each={move || planner.get().characters().get(&character_uuid).iter().flat_map(|c| c.spells().iter().map(|s| s.uuid)).collect::<Vec<_>>()}
                            key=|spell_uuid| *spell_uuid
                            children=move |spell_uuid| {
                                move || {
                                    let planner = planner.read();
                                    let character = planner.characters().get(&character_uuid)?;
                                    let spell = character.spells().get(&spell_uuid)?;

                                    Some(view! {
                                        <CharacterSpellToggle character_uuid spell={spell.clone()} />
                                    })
                                }
                            }
                        />
                    </div>
                }
            }
        />
    }
}