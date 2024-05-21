use leptos::prelude::*;

use fight_domain::AttackUuid;
use optimizer::AssignmentState;

use crate::api::icon_url;
use crate::api::ui_assignment::UiAssignment;
use crate::api::ui_character::UiCharacter;
use crate::api::ui_spell::UiSpell;
use crate::api::ui_state::UiState;
use crate::reactive::ForEach;

#[component]
pub fn Assignments() -> impl IntoView {
    let ui_state = use_context::<UiState>().unwrap();
    view! {
        <ForEach
            each=Signal::derive(move || ui_state.all_assignments())
            children=move |group| {
                let ((character_uuid, attack_uuid), assignments) = group;
                let column = format!("character_{}", character_uuid);
                let row = format!("attack_{}", attack_uuid);
                let attack_timer = move || {
                    ui_state.attacks().get(&attack_uuid).as_ref().unwrap().timer.static_timer()
                };
                view! {
                    <div
                        class="inline-flex justify-center items-center flex-wrap"
                        style:grid-column-start=column
                        style:grid-row-start=row
                    >
                        <ForEach
                            each=move || assignments.clone()
                            children=move |assignment| {
                                let is_assignable = Signal::derive({
                                    let assignment = assignment.clone();
                                    move || ui_state.is_spell_assignable(assignment.clone())
                                });
                                move || {
                                    let ui_character = ui_state
                                        .ui_characters()
                                        .get(&character_uuid)
                                        .cloned()
                                        .unwrap();
                                    let spell = ui_character
                                        .spells
                                        .get(&assignment.spell)
                                        .cloned()
                                        .unwrap();
                                    view! {
                                        <AssignmentIcon
                                            ui_character
                                            attack=attack_uuid
                                            spell
                                            assignment=assignment.clone()
                                            is_assignable
                                        />
                                    }
                                }
                            }
                        />

                    </div>
                }
            }
        />
    }
}

#[component]
fn AssignmentIcon(
    #[prop(into)] ui_character: UiCharacter,
    #[prop(into)] attack: AttackUuid,
    #[prop(into)] spell: UiSpell,
    #[prop(into)] assignment: UiAssignment,
    #[prop(into)] is_assignable: Signal<bool>,
) -> impl IntoView {
    let ui_state = use_context::<UiState>().unwrap();
    let src = icon_url(spell.identifier);
    let is_assigned = move || assignment.state != AssignmentState::Unassigned;
    let is_not_assigned = move || !is_assigned();
    let is_not_forced = move || assignment.state != AssignmentState::Forced;
    let is_forced = move || assignment.state == AssignmentState::Forced;
    let tag = spell.icon_text.unwrap_or("\u{00A0}".to_string());
    let background_image = match src {
        Some(src) => format!("url(\"{src}\")"),
        None => "".to_string(),
    };
    let is_minor = spell.minor;

    view! {
        <a
            class="z-20 inline-flex justify-center items-center \
            bg-cover bg-center bg-no-repeat bg-clip-border bg-origin-border \
            rounded-md \
            transition \
            border border-black \
            w-12 h-8 m-px \
            text-shadow-outline shadow-black font-bold \
            select-none cursor-pointer"
            style:background-image=background_image
            on:click=move |_| {
                ui_state.toggle_assignment(&assignment);
            }

            class=("w-8", is_minor)
            class=("hover:brightness-125", move || is_assigned() && is_not_forced())
            class=("border-0", move || is_assigned() && is_not_forced())
            class=("opacity-50", move || is_assigned() && is_not_forced())
            class=("opacity-5", is_not_assigned)
            class=("blur-xs", is_not_assigned)
            class=("contrast-300", is_not_assigned)
            class=("grayscale", is_not_assigned)
            class=("brightness-75", is_not_assigned)
            class=("hover:opacity-20", is_not_assigned)
            class=("hover:brightness-100", is_not_assigned)
            class=("hover:grayscale-0", is_not_assigned)
            class=("hover:contrast-100", is_not_assigned)
            class=("border-red-700", move || !is_assignable() && is_forced())
            class=("border-3", move || !is_assignable() && is_forced())
            class=("hover:border-red-700", move || !is_assignable())
            class=("hover:border-3", move || !is_assignable())
        >
            {tag}
        </a>
    }
}
