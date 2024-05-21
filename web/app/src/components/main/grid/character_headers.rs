use convert_case::{Case, Casing};
use itertools::Itertools;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use auto_battle_net::{Locale, LocalizedString};
use auto_battle_net::profile::character_profile::character_profile_summary::CharacterProfileSummaryRequest;

use crate::api::fuzzy_search::{fuzzy_score, fuzzy_search, Searchable, Term};
use crate::api::ui_character::{UiCharacter, UiCharacterTemplate};
use crate::api::ui_state::UiState;
use crate::components::AutocompleteDropdown;
use crate::context::*;
use crate::misc::flatten_ok::FlattenOk;
use crate::misc::localized_string_with_context::LocalizedStringWithContext;
use crate::reactive::async_ext::ReadyOrReloading;
use crate::reactive::ForEach;
use crate::serverfns::realms_for_character;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RosterEntry {
    pub name: String,
    pub realm: Option<PlannerRealm>,
    pub class: PlayerClass,
}

impl Searchable for RosterEntry {
    fn search_terms(&self) -> Vec<Term> {
        self.name.search_terms()
    }
}

#[component]
pub fn CharacterHeaders() -> impl IntoView {
    let ui_state = use_context::<UiState>().unwrap();
    view! {
        <ForEach
            each=move || ui_state.ui_characters()
            children=move |ui_character| {
                let uuid = ui_character.uuid;
                let replace_this_character = move |new_character| {
                    ui_state.replace_ui_character(uuid, new_character);
                };
                let remove_this_character = move || {
                    ui_state.remove_ui_character(uuid);
                };
                view! {
                    <CharacterHeader
                        ui_character=ui_character
                        replace_ui_character=replace_this_character
                        remove_ui_character=remove_this_character
                    />
                }
            }
        />
    }
}

#[component]
pub fn CharacterHeader<G, H>(
    #[prop(into)] ui_character: UiCharacter,
    replace_ui_character: G,
    remove_ui_character: H,
) -> impl IntoView
where
    G: Fn(UiCharacterTemplate) + Copy + Send + Sync + 'static,
    H: Fn() + Copy + 'static,
{
    let ui_state = use_context::<UiState>().unwrap();
    let uuid = ui_character.uuid;
    let class_color = Signal::derive({
        let ui_character = ui_character.clone();
        move || {
            if let Some(class) = &ui_character.class {
                let class_name = class.get(Locale::EnglishUnitedStates);
                format!("cc-{}", class_name.to_case(Case::Kebab))
            } else {
                "cc-general".to_string()
            }
        }
    });

    let is_editable = ui_character.editable;
    let on_blur = {
        let is_name_none = ui_character.name.is_none();
        move || {
            ui_state.set_ui_character_editing(uuid, false);
            if is_name_none {
                ui_state.remove_ui_character(uuid);
            }
        }
    };
    let editing = move || ui_state.ui_character_editing(uuid);
    let (picking, set_picking) = signal(false);

    let class_specs = Signal::<Vec<(LocalizedString, Option<LocalizedString>)>>::derive({
        let ui_character = ui_character.clone();
        move || {
            let class_spec_index = use_context::<ClassSpecIndex>().unwrap();
            if let Some(class) = &ui_character.class {
                class_spec_index
                    .specs_for_class(class.clone())
                    .get()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|spec| (class.clone(), Some(spec)))
                    .collect()
            } else {
                class_spec_index
                    .classes()
                    .get()
                    .into_iter()
                    .map(|class| (class, None))
                    .collect()
            }
        }
    });

    view! {
        <div
            class="relative mt-4 flex h-14 w-full sm:mt-0"
            class=("min-w-[8rem]", editing)
            style:grid-row-start="character_name"
            style:grid-column-start=format!("character_{}", uuid)
        >
            <div class="-ml-6 flex">
                <div
                    class="absolute flex h-12 w-full items-start justify-start text-slate-300"
                    class=("overflow-x-hidden", move || !editing())
                >
                    <div class="-mr-0.5 flex h-full flex-col items-center">
                        <button
                            class=move || {
                                format!("focus-visible:border-{}", class_color())
                                    + " mr-0.5 flex-grow items-center flex rounded-l-md border border-transparent \
                                                                px-1 text-slate-300 hover:text-white focus-visible:outline-none"
                            }

                            class=("cursor-default", move || !is_editable)
                            on:click=move |_| remove_ui_character()
                        >
                            <Show when=move || is_editable fallback=|| ()>
                                <div class="fa-solid fa-xmark text-sm"></div>
                            </Show>
                        </button>
                        <button
                            class=move || {
                                format!("focus-visible:border-{}", class_color())
                                    + " mr-0.5 flex-grow items-center flex rounded-l-md border border-transparent \
                                        px-1 text-slate-300 hover:text-white focus-visible:outline-none \
                                        z-20"
                            }

                            class=("cursor-default", move || !is_editable)
                            on:click=move |_| {
                                set_picking
                                    .update(|picking| {
                                        *picking = !*picking;
                                    });
                            }
                        >

                            <Show when=move || is_editable && !editing() fallback=|| ()>
                                <div
                                    class="fa-solid text-xs"
                                    class:fa-chevron-up=picking
                                    class:fa-chevron-down=move || !picking()
                                ></div>
                            </Show>
                        </button>
                    </div>

                    {
                        let ui_character = ui_character.clone();
                        view! {
                            <Show when=move || !editing() fallback=|| ()>
                                <CharacterHeaderNormal
                                    ui_character=ui_character.clone()
                                    class_color=class_color
                                    on_click_name=move || {
                                        if is_editable {
                                            ui_state.set_ui_character_editing(uuid, true);
                                        }
                                    }

                                    on_click_spec=move || {
                                        if is_editable {
                                            set_picking(true);
                                        }
                                    }

                                    picking=picking
                                    editable=is_editable
                                />
                            </Show>
                        }
                    }

                    <Show when=editing fallback=|| ()>
                        <CharacterHeaderEditing
                            current_ui_character=ui_character.clone()
                            replace_ui_character=replace_ui_character
                            on_blur=on_blur
                        />
                    </Show>
                </div>
                <Show when=picking fallback=|| ()>
                    <ul class="absolute z-30 mt-6 rounded-md border-2 border-slate-950 bg-slate-800 text-left text-sm font-normal text-slate-300 cursor-pointer">
                        <For
                            each=move || {
                                class_specs()
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, class_spec)| (index == 0, class_spec))
                            }

                            key=move |(_, class_spec)| class_spec.clone()
                            children=move |(is_first, (class, spec))| {
                                let class2 = class.clone();
                                let class3 = class.clone();
                                let spec2 = spec.clone();
                                view! {
                                    <li
                                        class="flex w-full hover:bg-slate-700"
                                        on:click={
                                            let class = class.clone();
                                            let spec = spec.clone();
                                            move |_| {
                                                ui_state.set_ui_character_class(uuid, Some(class.clone()));
                                                ui_state.set_ui_character_spec(uuid, spec.clone());
                                                if spec.is_some() {
                                                    set_picking(false);
                                                }
                                            }
                                        }
                                    >

                                        <div class=move || {
                                            format!(
                                                "border-r-cc-{}",
                                                class2.get(Locale::EnglishUnitedStates).to_case(Case::Kebab),
                                            ) + " w-5 box-content border-r-2"
                                        }></div>
                                        <div
                                            class="flex-grow pl-1 pr-2 whitespace-nowrap"
                                            class=("border-t", !is_first)
                                            class=("border-slate-950", !is_first)
                                        >
                                            {move || {
                                                spec2.as_ref().map(|s| format!("{} ", s.localize()))
                                            }}

                                            {move || class3.localize().to_string()}
                                        </div>
                                    </li>
                                }
                            }
                        />

                    </ul>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn CharacterHeaderNormal<F, G>(
    #[prop(into)] ui_character: UiCharacter,
    #[prop(into)] class_color: Signal<String>,
    #[prop(into)] picking: Signal<bool>,
    editable: bool,
    on_click_name: F,
    on_click_spec: G,
) -> impl IntoView
where
    F: Fn() + 'static,
    G: Fn() + Copy + Send + Sync + 'static,
{
    let character_name = ui_character.name;
    let character_realm = Signal::derive({
        let ui_character_realm = ui_character.realm.clone();
        move || {
            let user_realm = use_context::<PlannerContext>()
                .unwrap()
                .user()
                .ready_or_reloading()
                .flatten()
                .map(|u| u.realm.name);
            let character_realm = ui_character_realm.as_ref().map(|r| &r.name);
            if user_realm.as_ref() != character_realm {
                character_realm.cloned()
            } else {
                None
            }
        }
    });
    let character_spec = StoredValue::new(format!(
        "\u{200B}{}{}",
        ui_character
            .spec
            .map(move |s| s.localize().to_string())
            .unwrap_or_default(),
        ui_character
            .class
            .map(move |s| format!(" {}", s.localize()))
            .unwrap_or_default()
    ));

    view! {
        <div class=move || {
            format!("border-{}", class_color())
                + " flex h-full flex-col items-center justify-around overflow-x-hidden border-l-2 transition-colors"
        }>
            <span
                class="w-full overflow-x-hidden text-ellipsis whitespace-nowrap pl-1 text-left"
                tabindex="-1"
            >
                <span
                    class="group"
                    on:click=move |_| {
                        on_click_name();
                    }
                >

                    <span
                        class=("cursor-text", editable)
                        class=("group-hover:text-white", editable)
                    >
                        {character_name}
                    </span>
                    {move || {
                        character_realm()
                            .map(|realm| {
                                view! {
                                    <span
                                        class="text-slate-400 text-sm"
                                        class=("cursor-text", editable)
                                        class=("group-hover:text-slate-300", editable)
                                    >
                                        {move || format!("-{}", realm.localize())}
                                    </span>
                                }
                            })
                    }}

                </span>
            </span>
            <span
                class="w-full overflow-x-hidden text-ellipsis whitespace-nowrap pl-1 text-left text-sm font-normal"
                tabindex="-1"
            >
                <Show when=move || !picking() fallback=|| "\u{200B}">
                    <span
                        on:click=move |_| {
                            on_click_spec();
                        }

                        class=("cursor-pointer", editable)
                        class=("hover:text-white", editable)
                    >
                        {character_spec.get_value()}
                    </span>
                </Show>
            </span>
        </div>
    }
}

#[component]
pub fn CharacterHeaderEditing<G, H>(
    #[prop(into)] current_ui_character: UiCharacter,
    replace_ui_character: G,
    on_blur: H,
) -> impl IntoView
where
    G: Fn(UiCharacterTemplate) + Copy + Send + Sync + 'static,
    H: Fn() + Copy + Send + Sync + 'static,
{
    let ui_state = use_context::<UiState>().unwrap();
    let autocomplete = {
        let guild_roster = use_context::<GuildRoster>().unwrap();
        let class_cache = use_context::<ClassCache>().unwrap();
        let planner_context = use_context::<PlannerContext>().unwrap();
        move |query: Signal<String>| {
            let realms_for_character = Resource::new_serde(
                move || (planner_context.region().get(), query()),
                move |(region, query)| async move {
                    if let Some((query_name, query_realm)) = query.split_once('-') {
                        realms_for_character(query_name.to_string(), region)
                            .await
                            .unwrap_or_default()
                    } else {
                        vec![]
                    }
                },
            );
            let hits = Memo::new({
                let guild_roster = guild_roster.clone();
                move |_| {
                    let query = query();
                    if let Some((query_name, query_realm)) = query.split_once('-') {
                        realms_for_character
                            .ready_or_reloading()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|rfc| {
                                let realm_name = rfc.realm.name.localize();
                                let score = fuzzy_score(query_realm, &realm_name.as_ref());

                                let character_name = query_name.to_case(Case::Train);

                                let entry = RosterEntry {
                                    name: character_name,
                                    realm: Some(PlannerRealm {
                                        name: rfc.realm.name,
                                        slug: rfc.realm.slug,
                                    }),
                                    class: PlayerClass::Known(rfc.class),
                                };
                                (score, entry)
                            })
                            .sorted_by_key(|(score, entry)| {
                                (
                                    -score,
                                    entry
                                        .realm
                                        .as_ref()
                                        .unwrap()
                                        .name
                                        .get(Locale::EnglishUnitedStates)
                                        .to_string(),
                                )
                            })
                            .map(|(_, entry)| entry)
                            .collect::<Vec<RosterEntry>>()
                    } else {
                        fuzzy_search(&query, guild_roster.clone().into_iter())
                            .collect::<Vec<RosterEntry>>()
                    }
                }
            });
            Signal::derive(move || {
                hits()
                    .into_iter()
                    .map(|e| {
                        if e.class == PlayerClass::Unknown {
                            let class = class_cache.get(&e.name, &e.realm.as_ref().unwrap().name);
                            RosterEntry {
                                name: e.name.clone(),
                                realm: e.realm.clone(),
                                class: class.get(),
                            }
                        } else {
                            e
                        }
                    })
                    .filter(|e| match e.class {
                        PlayerClass::Known(_) => true,
                        PlayerClass::Missing => false,
                        PlayerClass::Unknown => false,
                    })
                    .filter(|e| {
                        !ui_state
                            .ui_characters()
                            .into_iter()
                            .filter(|c| c.uuid != current_ui_character.uuid)
                            .any(|c| c.name.as_ref() == Some(&e.name) && c.realm == e.realm)
                    })
                    .collect::<Vec<RosterEntry>>()
            })
        }
    };

    let (selected_entry, set_selected_entry) = signal::<Option<RosterEntry>>(None);
    Effect::new(move |_| {
        if let Some(entry) = selected_entry() {
            if let PlayerClass::Known(class) = entry.class {
                let class_spec_index = use_context::<ClassSpecIndex>().unwrap();
                let specs = class_spec_index
                    .specs_for_class(class.clone())
                    .get()
                    .unwrap_or_default();
                let first_spec = specs.last().cloned();
                if let Some(first_spec) = first_spec {
                    let ui_character = UiCharacterTemplate::new_known(
                        entry.name.clone(),
                        entry
                            .realm
                            .or_else(move || {
                                let user = use_context::<PlannerContext>().unwrap().user();
                                user.ready_or_reloading().flatten().map(|u| u.realm)
                            })
                            .unwrap(),
                        class,
                        first_spec,
                    );
                    replace_ui_character(ui_character);
                }
            }
        }
    });

    view! {
        <Suspense fallback=|| ()>
            <AutocompleteDropdown
                autocomplete=autocomplete.clone()
                key=move |entry| {
                    format!(
                        "{}-{}",
                        &entry.name,
                        entry.realm.as_ref().map(|r| r.slug.clone()).unwrap_or_default(),
                    )
                }

                view=move |entry, query, is_selected| {
                    let planner_context = use_context::<PlannerContext>().unwrap();
                    let region = Memo::new(move |_| planner_context.region().get());
                    let class = entry.class.clone();
                    let class_border = Signal::derive({
                        let class = class.clone();
                        move || {
                            class
                                .opt()
                                .map(|c| {
                                    format!(
                                        "border-cc-{}",
                                        c.get(Locale::EnglishUnitedStates).to_case(Case::Kebab),
                                    )
                                })
                                .unwrap_or("border-transparent".to_string())
                        }
                    });
                    let name = Signal::derive({
                        let query = query.to_string();
                        move || {
                            let realm = if query.contains('-') {
                                entry
                                    .realm
                                    .as_ref()
                                    .map(|r| format!("-{}", r.name.localize()))
                                    .unwrap_or_default()
                            } else {
                                "".to_string()
                            };
                            entry.name.clone() + &realm
                        }
                    });
                    let class = Signal::derive(move || { class.opt().map(|c| c.localize()) });
                    view! {
                        <div
                            class="flex w-full text-slate-300 items-center px-1 hover:!bg-slate-700"
                            class=("bg-slate-700", is_selected)
                            class=("group-hover:bg-slate-800", is_selected)
                        >
                            <Suspense>
                                <div class=move || {
                                    format!("{} my-1 border-l-2 mr-1 text-xs", class_border())
                                }>"\u{200b}"</div>
                                <span class="mr-1 flex-grow overflow-x-hidden text-ellipsis text-left">
                                    {name}
                                </span>
                                <span class="text-xs font-light text-slate-400">
                                    {move || class.get().map(|s| s.to_string())}
                                </span>
                            </Suspense>
                        </div>
                    }
                }

                view_custom=move |query, is_selected| {
                    view! {
                        <div
                            class="flex w-full text-slate-300 items-center px-1 hover:!bg-slate-700"
                            class=("bg-slate-700", is_selected)
                            class=("group-hover:bg-slate-800", is_selected)
                        >
                            <div class="my-1 border-l-2 border-transparent text-xs"></div>
                            <span class="mr-1 flex-grow overflow-x-hidden text-ellipsis text-left">
                                {query.to_case(Case::Train)}
                            </span>
                            <span class="text-xs font-light text-slate-400">"Unknown"</span>
                        </div>
                    }
                }

                on_select=move |entry: &RosterEntry| {
                    set_selected_entry(Some(entry.clone()));
                }

                on_select_custom=move |query| {
                    let ui_character = UiCharacterTemplate::new_custom(query.to_case(Case::Train));
                    replace_ui_character(ui_character);
                }

                on_blur=on_blur
            />
        </Suspense>
    }
}
