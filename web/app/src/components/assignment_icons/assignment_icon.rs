use std::time::Duration;
use leptos::prelude::*;

use fight_domain::{AttackUuid, CharacterUuid, Spell};
use optimizer::{AssignmentState};

use crate::api::icon_url;
use crate::context::use_planner;

#[component]
pub fn AssignmentIcon(
    character_uuid: CharacterUuid,
    attack_uuid: AttackUuid,
    spell: Spell,
    assignment_state: AssignmentState,
    is_assignable: bool,
) -> impl IntoView {
    let planner = use_planner();

    let src = icon_url(spell.identifier);
    let is_assigned = assignment_state != AssignmentState::Unassigned;
    let is_not_assigned = !is_assigned;
    let is_not_forced = assignment_state != AssignmentState::Locked;
    let is_forced = assignment_state == AssignmentState::Locked;
    let tag = spell.icon_text.unwrap_or("\u{00A0}".to_string());
    let background_image = match src {
        Some(src) => format!("url('{src}')"),
        None => "".to_string(),
    };
    let is_minor = spell.minor;

    let clicked = RwSignal::new(false);
    Effect::new(move |_| {
        if clicked.get() {
            set_timeout(move || {
                clicked.set(false);
            }, Duration::from_millis(200));
        }
    });

    view! {
        <a
            class="flex justify-center items-center \
            bg-cover bg-center bg-no-repeat bg-clip-border bg-origin-border \
            rounded-md \
            transition \
            border border-black \
            w-12 h-8 m-px \
            text-shadow-outline shadow-black font-bold \
            select-none cursor-pointer"
            style:background-image=background_image
            on:mousedown=move |ev| {
                if ev.button() != 0 {
                    return;
                }
                clicked.set(true);
                if is_assignable {
                    planner.update(|planner| {
                        planner.toggle_assignment(character_uuid, spell.uuid, attack_uuid);
                    });
                }
            }

            class=("w-8", is_minor)
            class=("hover:brightness-125", is_assigned && is_not_forced)
            class=("border-0", is_assigned && is_not_forced)
            class=("opacity-20", is_assigned && is_not_forced)
            class=("opacity-5", is_not_assigned)
            class=("blur-xs", is_not_assigned)
            class=("contrast-300", is_not_assigned)
            class=("grayscale", is_not_assigned)
            class=("brightness-75", is_not_assigned)
            class=("hover:opacity-75", is_not_forced)
            class=("hover:brightness-100", is_not_assigned)
            class=("hover:grayscale-0", is_not_assigned)
            class=("hover:contrast-100", is_not_assigned)
            class=("border-red-700", !is_assignable && is_forced)
            class=("border-3", !is_assignable && is_forced)
            class=("hover:border-red-700", !is_assignable)
            class=("hover:border-3", !is_assignable)
            class=("hover:!opacity-100", move || clicked.get())
        >
            {tag}
        </a>
    }
}
