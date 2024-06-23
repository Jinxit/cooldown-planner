use crate::api::icon_url;
use crate::api::ui_character::UiCharacter;
use crate::api::ui_state::UiState;
use crate::reactive::ForEach;
use fight_domain::{Lookup, Spell, SpellUuid};
use leptos::*;
use std::sync::Arc;
use fight_domain::LookupKey;


#[component]
pub fn CharacterSpells() -> impl IntoView {
    let ui_state = use_context::<UiState>().unwrap();
    view! {
        <For
            each=move || ui_state.ui_characters()
            // TODO: this is wrong
            key=move |value| value.lookup_key()
            children=move |ui_character| {
                let column = move || format!("character_{}", ui_character.uuid);
                let row = "character_spells";
                view! {
                    <div
                        class="mb-4 inline-flex justify-center items-center flex-wrap z-10"
                        style:grid-column-start=column
                        style:grid-row-start=row
                    >
                        <CharacterSpellsToggles ui_character=ui_character/>
                    </div>
                }
            }
        />
    }
}

#[component]
fn CharacterSpellsToggles(ui_character: UiCharacter) -> impl IntoView {
    let ui_state = use_context::<UiState>().unwrap();
    view! {
        <ForEach
            each=move || ui_character.spells.clone()
            children=move |spell| {
                let toggle_spell = move || {
                    ui_state.toggle_spell_enabled(ui_character.uuid, spell.uuid);
                };
                view! { <CharacterSpellsToggle spell=spell toggle_spell=toggle_spell/> }
            }
        />
    }
}

#[component]
fn CharacterSpellsToggle<F>(#[prop(into)] spell: Spell, toggle_spell: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    let src = icon_url(spell.identifier);
    let tag = spell.icon_text.unwrap_or("\u{00A0}".to_string());
    let background_image = match src {
        Some(src) => format!("url(\"{src}\")"),
        None => "".to_string(),
    };

    view! {
        <a
            class="inline-flex justify-center items-center \
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
            on:click=move |_| {
                toggle_spell();
            }
        >

            {tag}
        </a>
    }
}
