use leptos::prelude::*;

use fight_domain::{CharacterUuid, Spell};

use crate::api::icon_url;
use crate::context::use_planner;

#[component]
pub fn CharacterSpellToggle(character_uuid: CharacterUuid, spell: Spell) -> impl IntoView {
    let planner = use_planner();

    let src = icon_url(spell.identifier);
    let tag = spell.icon_text.unwrap_or("\u{00A0}".to_string());
    let background_image = match src {
        Some(src) => format!("url('{src}')"),
        None => "".to_string(),
    };

    view! {
        <button
            class="justify-center items-center \
            bg-cover bg-center bg-no-repeat bg-clip-border bg-origin-border \
            rounded-md \
            transition \
            border border-black \
            h-8 m-px \
            text-shadow-outline shadow-black font-bold \
            select-none cursor-pointer \
            hover:border-slate-600"
            class=("w-12", !spell.minor)
            class=("w-8", spell.minor)
            class=("hover:brightness-125", spell.enabled)
            class=("grayscale", !spell.enabled)
            class=("brightness-75", !spell.enabled)
            class=("hover:brightness-100", !spell.enabled)
            style:background-image=background_image
            on:mousedown=move |ev| {
                if ev.button() != 0 {
                    return;
                }
                planner.update(|planner| {
                    planner.toggle_spell_enabled(character_uuid, spell.uuid);
                });
            }
        >
            {tag}
        </button>
    }
}
