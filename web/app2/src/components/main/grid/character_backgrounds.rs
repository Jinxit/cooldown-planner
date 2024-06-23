use std::collections::HashMap;
use std::sync::Arc;

use leptos::*;
use tracing::error;
use url::Url;

use auto_battle_net::profile::character_profile::character_profile_summary::CharacterProfileSummaryResponse;
use i18n::{Locale, Region};

use crate::api::ui_character::UiCharacter;
use crate::api::ui_state::UiState;
use crate::context::PlannerContext;
use crate::misc::flatten_ok::FlattenOk;
use crate::reactive::ForEach;
use crate::serverfns::character_main_image;
use fight_domain::LookupKey;


#[component]
pub fn CharacterBackgrounds(#[prop(into)] tab_open: Signal<bool>) -> impl IntoView {
    let ui_state = use_context::<UiState>().unwrap();
    view! {
        <For
            each=move || ui_state.ui_characters()
            // TODO: this is wrong
            key=move |value| value.lookup_key().clone()
            children=move |ui_character| {
                view! { <CharacterBackground ui_character tab_open/> }
            }
        />
    }
}

#[component]
pub fn CharacterBackground(
    ui_character: UiCharacter,
    #[prop(into)] tab_open: Signal<bool>,
) -> impl IntoView {
    let default_offset = (-48.0, -20.0);
    let offset_map = StoredValue::new(Arc::new(
        [
            (("Blood Elf", "MALE"), (18.0, -23.2)),
            (("Blood Elf", "FEMALE"), (18.0, -26.0)),
            (("Dracthyr", "FEMALE"), (18.0, -19.3)),
            (("Dracthyr", "MALE"), (18.0, -19.3)),
            (("Goblin", "MALE"), (18.0, -34.0)),
            (("Gnome", "MALE"), (18.0, -35.0)),
            (("Gnome", "FEMALE"), (18.0, -36.0)),
            (("Highmountain Tauren", "FEMALE"), (18.0, -21.0)),
            (("Highmountain Tauren", "MALE"), (18.0, -26.5)),
            (("Kul Tiran", "MALE"), (18.0, -20.0)),
            (("Night Elf", "FEMALE"), (18.0, -22.6)),
            (("Nightborne", "FEMALE"), (18.0, -22.2)),
            (("Orc", "MALE"), (18.0, -25.5)),
            (("Pandaren", "MALE"), (18.0, -24.0)),
            (("Tauren", "FEMALE"), (18.0, -21.0)),
            (("Tauren", "MALE"), (18.0, -26.5)),
            (("Troll", "FEMALE"), (18.0, -20.0)),
            (("Undead", "FEMALE"), (18.0, -27.0)),
            (("Undead", "MALE"), (19.0, -27.5)),
            (("Vulpera", "FEMALE"), (18.0, -34.0)),
            (("Zandalari Troll", "FEMALE"), (18.0, -20.5)),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>(),
    ));

    let planner_context = use_context::<PlannerContext>().unwrap();
    let user = planner_context.user.clone();
    let region = planner_context.region.clone();

    let realm_slug = Signal::derive({
        let realm = ui_character.realm.clone();
        move || {
            realm
                .clone()
                .or_else(|| Some(user.get()?.realm))
                .map(|r| r.slug)
                .unwrap_or_default()
        }
    });

    #[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
    struct BackgroundData {
        image_url: Url,
        character_summary: CharacterProfileSummaryResponse,
    }
    let background_data = Resource::new(move || (realm_slug(), region.get()), {
        let character_name = ui_character.name.unwrap_or_default();
        move |(realm_slug, region): (String, Region)| {
            let character_name = character_name.clone();
            let realm_slug = realm_slug.clone();
            async move {
                if ui_character.is_general || character_name.is_empty() {
                    None
                } else {
                    let image_url =
                        character_main_image(character_name.clone(), realm_slug.clone(), region)
                            .await;
                    /*let character_summary =
                        character_summary(character_name.clone(), realm_slug.clone(), region);
                    let (image_url, character_summary) = try_join!(image_url, character_summary).ok()?;
                    Some(BackgroundData {
                        image_url,
                        character_summary,
                    })*/
                    None
                }
            }
        }
    });

    let background_view = move |background_data: &BackgroundData| {
        let BackgroundData {
            image_url,
            character_summary,
        } = background_data;
        let background_image = format!("url({image_url})");
        let offset_map = offset_map.get_value();
        let race_name = character_summary.race.name.get(Locale::EnglishUnitedStates);
        let offset = offset_map.get(&(race_name, character_summary.gender.r#type.as_ref()));
        if offset.as_ref().is_none() {
            error!(
                "Unknown race {:?} and gender {:?}",
                &character_summary.race.name, &character_summary.gender.name
            );
        }
        let offset = offset.unwrap_or(&default_offset);
        let bg_offset = format!("right calc(34% - {}rem) top {}rem", offset.0, offset.1);
        view! {
            <div
                class="fade-to-b-1200 absolute h-full w-full opacity-25 brightness-0 contrast-100"
                style:background-position=bg_offset.clone()
                style:background-image=background_image.clone()
            ></div>
            <div
                class="fade-to-b-4rem absolute h-full w-full opacity-75"
                style:background-position=bg_offset
                style:background-image=background_image
            ></div>
            <div class="absolute h-full w-full bg-gradient-to-t from-slate-700 to-transparent to-60%"></div>
            <div class="absolute h-full w-full bg-gradient-to-l from-slate-700 to-transparent to-10%"></div>
            <div class="absolute h-full w-full bg-gradient-to-r from-slate-700 to-transparent to-[8rem]"></div>
        }
    };

    view! {
        <div
            class="relative -ml-2 h-full pointer-events-none @container transition-[margin]"
            class=("-mt-4", tab_open)
            class=("-mt-8", move || !tab_open())
            style:grid-column-start=move || format!("character_{}", &ui_character.uuid)
            style:grid-row-end="10000"
            style:grid-row-start="character_name"
            style:width="calc(100% + 1rem)"
        >
            <div class="[10rem]:opacity-100 w-full opacity-0 @[6rem]:opacity-50 @[8rem]:opacity-75">
                <Suspense>
                    {move || background_data
                        .get()
                        .flatten()
                        .map(move |background_data| { background_view(&background_data) })
                    }
                </Suspense>
            </div>
        </div>
    }
}
