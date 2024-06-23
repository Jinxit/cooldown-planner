use leptos::prelude::*;
use planner::Assignability;
use crate::components::assignment_icons::assignment_icon::AssignmentIcon;
use crate::context::use_planner;

#[component]
pub fn AssignmentIcons() -> impl IntoView {
    let planner = use_planner();
    view! {
        <For
            each={move || planner.get().attacks().iter().map(|a| a.uuid).collect::<Vec<_>>()}
            key=|uuid| *uuid
            let:attack_uuid
        >
            <For
                each={move || planner.get().characters().iter().map(|c| c.uuid).collect::<Vec<_>>()}
                key=|uuid| *uuid
                let:character_uuid
            >
                <div
                    class="flex justify-center items-center flex-wrap"
                    style:grid-column-start=format!("character_{}", character_uuid)
                    style:grid-row-start=format!("attack_{}", attack_uuid)
                >
                    <For
                        each={move || planner.get().characters().get(&character_uuid).iter().flat_map(|c| c.spells().iter().map(|s| s.uuid)).collect::<Vec<_>>()}
                        key=|uuid| *uuid
                        children=move |spell_uuid| {
                            move || {
                                let planner = planner.read();
                                let character = planner.characters().get(&character_uuid)?;
                                let spell = character.spells().get(&spell_uuid)?;
                                let assignment_state = character.assignment_state(spell_uuid, attack_uuid);
                                let assignability = planner.is_spell_assignable(character_uuid, spell.uuid, attack_uuid);
                                let is_assignable = assignability == Assignability::Assignable;
                                Some(view! {
                                    <AssignmentIcon character_uuid attack_uuid spell={spell.clone()} assignment_state is_assignable />
                                })
                            }
                        }
                    />
                </div>
            </For>
        </For>
    }
}