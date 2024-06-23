use convert_case::{Case, Casing};
use itertools::Itertools;
use leptos::html::Input;
use leptos::prelude::*;

use fight_domain::CharacterUuid;
use i18n::Locale;
use planner::{PlannerCharacterTemplate, PlannerRealm};
use planner::fuzzy_search::{fuzzy_score, fuzzy_search};
use planner::specs::default_spec_for_class;

use crate::components::autocomplete::*;
use crate::components::character_headers::class_color_bar::ClassColorBar;
use crate::context::{GuildRoster, RosterEntry, use_class_spec_index, use_planner, UserContext};
use crate::misc::localized_string_with_context::LocalizedStringWithContext;
use crate::serverfns::realms_for_character;

#[component]
pub fn NameEditing(
    current_character_uuid: CharacterUuid,
    editing: RwSignal<bool>,
) -> impl IntoView {
    let planner = use_planner();

    let query = RwSignal::new(String::new());
    let selection = RwSignal::new(0usize);

    let autocomplete_results = autocomplete(query);
    let user = use_context::<UserContext>().unwrap();
    let main_character = user.main_character;
    let csi = use_class_spec_index();

    let input_ref = NodeRef::<Input>::new();
    Effect::new(move |_| {
        if let Some(input_ref) = input_ref.try_get().flatten() {
            request_animation_frame(move || {
                let _ = input_ref.focus();
            });
        };
    });

    view! {
        <Autocomplete
            on_blur=Callback::new(move |_| {
                //planner.update(|planner| {
                //    planner.remove_character(current_character_uuid);
                //})
            })
            {..}
            class="flex flex-col overflow-y-visible min-w-32 pl-1">
            <AutocompleteInput
                node_ref={input_ref}
                {..}
                type="text"
                class="w-full rounded-md border-2 border-slate-500 bg-slate-900 px-1 text-slate-300 focus-visible:outline-none"
                prop:value=query

                on:input=move |ev| {
                    query.set(
                        event_target_value(&ev)
                            .chars()
                            .filter(|c| c.is_alphabetic() || *c == '-')
                            .collect::<String>()
                    );
                }
            />
            <Suspense>
                <AutocompleteItems {..}
                    class="flex flex-col divide-y divide-slate-900 rounded-b-md border-2 border-t-0 border-slate-900 bg-slate-800 cursor-pointer"
                >
                    <For
                        each={move || autocomplete_results.get().unwrap_or_default()}
                        key={|entry| format!("{}-{}-{}", entry.name, entry.realm.slug, entry.class.get(Locale::EnglishUnitedStates))}
                        children=move |entry| {
                            Suspend(async move {
                                let spec_name = default_spec_for_class(entry.class.get(Locale::EnglishUnitedStates));
                                let specs = csi.specs_for_class(entry.class.clone()).await;
                                let full_spec_name = specs.iter().find(|s| Some(s.get(Locale::EnglishUnitedStates)) == spec_name).cloned();

                                view! {
                                    <AutocompleteItem
                                        on_select=Callback::new({
                                            let entry = entry.clone();
                                            move |_| {
                                                let full_spec_name = full_spec_name.clone();
                                                planner.update(|planner| {
                                                    let new_uuid = planner.replace_character(current_character_uuid, PlannerCharacterTemplate::Known {
                                                        name: entry.name.clone(),
                                                        realm: entry.realm.clone(),
                                                        class: entry.class.clone(),
                                                    });
                                                    if let Some(full_spec_name) = full_spec_name {
                                                        planner.change_character_spec(new_uuid, full_spec_name.clone());
                                                    }
                                                });
                                            }
                                        })
                                        let:highlighted
                                        {..}
                                    >
                                        <div
                                            class="flex w-full flex-row items-center gap-1 px-1 items-center"
                                            class=("bg-slate-700", highlighted)
                                            class=("text-white", highlighted)
                                        >
                                            <ClassColorBar class={Some(entry.class.clone())} />
                                            <span class="flex-grow text-start">{entry.name.clone()}</span>
                                            <span class="text-xs font-light text-slate-400">{entry.realm.name.localize(user)}</span>
                                        </div>
                                    </AutocompleteItem>
                                }
                            })
                        }
                    />
                </AutocompleteItems>
            </Suspense>
        </Autocomplete>
    }
}

fn autocomplete(query: RwSignal<String>) -> AsyncDerived<Vec<RosterEntry>> {
    let user = use_context::<UserContext>().unwrap();
    let realms_for_character = realms_for_character_resource(query, user);
    let guild_roster = use_context::<GuildRoster>().unwrap();

    AsyncDerived::new(move || {
        let guild_roster = guild_roster.clone();
        async move {
            let query = query.get();
            if query.is_empty() {
                return vec![];
            }
            let realms_for_character_entries = realms_for_character.await;
            if !realms_for_character_entries.is_empty() {
                realms_for_character_entries
            } else {
                fuzzy_search(query, guild_roster.get().await.clone().into_iter()).take(10).collect()
            }
        }
    })
}

fn realms_for_character_resource(
    query: RwSignal<String>,
    user: UserContext,
) -> AsyncDerived<Vec<RosterEntry>> {
    let query_name = Memo::new(move |_| {
        query
            .get()
            .split_once('-')
            .map(|(name, _)| name.to_string())
    });
    let query_realm = Memo::new(move |_| {
        query
            .get()
            .split_once('-')
            .map(|(_, realm)| realm.to_string())
    });
    let rfc = Resource::new(
        move || query_name,
        move |query_name| async move {
            if let Some(query_name) = query_name.get() {
                realms_for_character(query_name.to_string(), user.region.await)
                    .await
                    .unwrap_or_default()
            } else {
                vec![]
            }
        },
    );
    let rfc_memo = Memo::new(move |_| rfc.get().unwrap_or_default().clone());
    AsyncDerived::new(move || async move {
        let locale = user.locale.await;
        if let Some((query_name, query_realm)) = query_name.get().zip(query_realm.get()) {
            let character_name = query_name.to_case(Case::Train);

            rfc_memo
                .get()
                .into_iter()
                .map(|rfc| {
                    let realm_name = rfc.realm.name.get(locale).to_string();
                    let score = fuzzy_score(&query_realm, &realm_name.as_ref());

                    let entry = RosterEntry {
                        name: character_name.clone(),
                        realm: PlannerRealm {
                            name: rfc.realm.name,
                            slug: rfc.realm.slug,
                        },
                        class: rfc.class,
                    };
                    (score, entry, realm_name)
                })
                .sorted_by_key(|(score, _, realm_as_string)| (-score, realm_as_string.clone()))
                .map(|(_, entry, _)| entry)
                .collect::<Vec<RosterEntry>>()
        } else {
            vec![]
        }
    })
}
