use crate::api::ui_character::UiCharacter;
use crate::api::ui_state::UiState;
use crate::context::PlannerContext;
use crate::misc::flatten_ok::FlattenOk;
use crate::reactive::blank_suspense::BlankSuspense;
use crate::reactive::resource_ext::ResourceMapExt;
use crate::reactive::{ForEach, ForLookup5};
use crate::serverfns::{character_main_image, character_summary};
use auto_battle_net::profile::character_profile::character_profile_summary::CharacterProfileSummaryResponse;
use auto_battle_net::{Locale, Region};
use fight_domain::Lookup;
use futures_util::try_join;
use leptos::*;
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;

#[component]
pub fn CharacterBackgrounds(#[prop(into)] tab_open: Signal<bool>) -> impl IntoView {
    let ui_state = expect_context::<UiState>();
    view! {
        <ForEach each=move || ui_state.ui_characters() bind:ui_character>
            <CharacterBackground ui_character tab_open/>
        </ForEach>
    }
}

#[component]
pub fn CharacterBackground(
    ui_character: UiCharacter,
    #[prop(into)] tab_open: Signal<bool>,
) -> impl IntoView {
    let default_offset = (-48.0, -20.0);
    let offset_map = store_value(Arc::new(
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

    let planner_context = expect_context::<PlannerContext>();

    let realm_slug = Signal::derive({
        let realm = ui_character.realm.clone();
        move || {
            realm
                .clone()
                .or_else(|| Some(planner_context.user.get()?.realm))
                .map(|r| r.slug)
                .unwrap_or_default()
        }
    });

    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    struct BackgroundData {
        image_url: Url,
        character_summary: CharacterProfileSummaryResponse,
    }
    let background_data = create_resource(move || (realm_slug(), planner_context.region.get()), {
        let character_name = ui_character.name.unwrap_or_default();
        move |(realm_slug, region): (String, Region)| {
            let character_name = character_name.clone();
            let realm_slug = realm_slug.clone();
            async move {
                if ui_character.is_general || character_name.is_empty() {
                    None
                } else {
                    let image_url =
                        character_main_image(character_name.clone(), realm_slug.clone(), region);
                    let character_summary =
                        character_summary(character_name.clone(), realm_slug.clone(), region);
                    let (image_url, character_summary) = try_join!(image_url, character_summary).ok()?;
                    Some(BackgroundData {
                        image_url,
                        character_summary,
                    })
                }
            }
        }
    });

    let background_view =
        move |background_data: &BackgroundData| {
            let BackgroundData {
                image_url,
                character_summary
            } = background_data;
            let background_image = format!("url({image_url})");
            let offset_map = offset_map();
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
            style:grid-row-end=10000
            style:grid-row-start="character_name"
            style:width="calc(100% + 1rem)"
        >
            <div class="[10rem]:opacity-100 w-full opacity-0 @[6rem]:opacity-50 @[8rem]:opacity-75">
                <BlankSuspense>
                    {move || {
                        background_data
                            .map(move |background_data| {
                                background_data.as_ref().map(|background_data| {
                                    background_view(
                                        background_data,
                                    )
                                })
                            })
                    }}
                </BlankSuspense>
            </div>
        </div>
    }
}
