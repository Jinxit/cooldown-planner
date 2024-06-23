use std::collections::HashMap;
use std::future::IntoFuture;
use std::sync::Arc;

use futures_util::try_join;
use leptos::prelude::*;
use tracing::error;
use url::Url;

use auto_battle_net::profile::character_profile::character_profile_summary::CharacterProfileSummaryResponse;
use fight_domain::{CharacterUuid, LookupKey};
use i18n::{Locale, LocalizedString, Region};
use planner::{PlannerCharacter, PlannerRealm};

use crate::context::{use_planner, UserContext};
use crate::misc::localized_string_with_context::LocalizedStringWithContext;
use crate::serverfns::{character_main_image, character_summary};

#[component]
pub fn CharacterBackgrounds(#[prop(into)] tab_open: Signal<bool>) -> impl IntoView {
    let planner = use_planner();
    view! {
        <For
            each={move || planner.get().characters().iter().filter(|c| !c.is_general()).map(|c| c.uuid).collect::<Vec<_>>()}
            key=|uuid| *uuid
            children=move |uuid| {
                let character = Memo::new(move |_| planner.read().characters().get(&uuid).cloned());
                // using Locale::EnglishUnitedStates here is fine because character names are constants anyways (apart from General which this isn't)
                let character_name = Memo::new(move |_| character.get().and_then(|c| c.name).map(|name| name.get(Locale::EnglishUnitedStates).to_string()));
                let character_realm = Memo::new(move |_| character.get().and_then(|c| c.realm).map(|realm| realm.clone()));
                move || {
                    let character_name = character_name.get()?;
                    let character_realm = character_realm.get()?;

                    Some(view! {
                        <CharacterBackground character_uuid=uuid character_name character_realm tab_open />
                    })
                }
            }
        />
    }
}

#[component]
pub fn CharacterBackground(
    character_uuid: CharacterUuid,
    character_name: String,
    character_realm: PlannerRealm,
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

    let user = use_context::<UserContext>().unwrap();
    let realm_slug = character_realm.slug;

    #[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
    struct BackgroundData {
        image_url: Url,
        character_summary: CharacterProfileSummaryResponse,
    }
    let background_data = Resource::new(|| (), {
        let character_name = character_name.clone();
        move |_| {
            let character_name = character_name.clone();
            let realm_slug = realm_slug.clone();
            async move {
                let region = user.region.await;
                let character_name = character_name.clone();
                let image_url = character_main_image(
                    character_name.clone(),
                    realm_slug.clone(),
                    region,
                );
                let character_summary = character_summary(
                    character_name.clone(),
                    realm_slug.clone(),
                    region,
                );
                let (image_url, character_summary) =
                    try_join!(image_url, character_summary).ok()?;
                Some(BackgroundData {
                    image_url,
                    character_summary,
                })
            }
        }
    });

    let background_view = async move {
        let BackgroundData {
            image_url,
            character_summary,
        } = background_data.await?;
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
        Some(view! {
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
        })
    };

    Some(view! {
        <div
            class="relative -ml-2 h-full pointer-events-none @container transition-[margin] -z-50"
            class=("-mt-4", tab_open)
            class=("-mt-8", move || !tab_open())
            style:grid-column-start=move || format!("character_{}", &character_uuid)
            style:grid-row-end="10000"
            style:grid-row-start="character_name"
            style:width="calc(100% + 1rem)"
        >
            <div class="[10rem]:opacity-100 w-full opacity-0 @[6rem]:opacity-50 @[8rem]:opacity-75">
                <Suspense>
                    {Suspend(background_view)}
                </Suspense>
            </div>
        </div>
    })
}
