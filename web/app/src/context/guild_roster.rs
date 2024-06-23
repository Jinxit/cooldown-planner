use std::future::IntoFuture;

use convert_case::{Case, Casing};
use leptos::prelude::*;
use leptos::prelude::guards::{AsyncPlain, ReadGuard};
use leptos::server::serializers::SerdeJson;
use serde::{Deserialize, Serialize};

use i18n::LocalizedString;
use planner::fuzzy_search::{Searchable, Term};
use planner::PlannerRealm;

use crate::context::UserContext;
use crate::serverfns::{PlayerClass, guild_roster};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RosterEntry {
    pub name: String,
    pub class: LocalizedString,
    pub realm: PlannerRealm,
}

impl Searchable for RosterEntry {
    fn search_terms(&self) -> Vec<Term> {
        self.name.search_terms()
    }
}

#[derive(Clone)]
pub struct GuildRoster(Resource<Vec<RosterEntry>>);

impl GuildRoster {
    pub fn new() -> Self {
        let user = use_context::<UserContext>().unwrap();
        GuildRoster(Resource::new(
            move || (),
            move |_| async move {
                let region = user.region.await;
                if let Some(mc) = user.main_character.await {
                    if let Some(guild) = mc.guild {
                        let entries =
                            guild_roster(mc.realm.slug.clone(), guild.to_case(Case::Kebab), region)
                                .await
                                .unwrap_or_else(|e| {
                                    tracing::error!("{:?}", e);
                                    vec![]
                                })
                                .into_iter()
                                .map(|gre| RosterEntry {
                                    name: gre.name,
                                    class: gre.class,
                                    realm: mc.realm.clone(),
                                })
                                .collect();
                        entries
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            },
        ))
    }

    pub fn get(&self) -> Resource<Vec<RosterEntry>> {
        self.0
    }
}

pub fn provide_guild_roster_context() {
    let roster = GuildRoster::new();
    provide_context(roster);
}
