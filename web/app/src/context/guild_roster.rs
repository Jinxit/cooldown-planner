use convert_case::{Case, Casing};
use leptos::prelude::*;
use leptos::server::serializers::SerdeJson;

use crate::components::main::RosterEntry;
use crate::context::PlannerContext;
use crate::reactive::async_ext::ReadyOrReloading;
use crate::serverfns::guild_roster;

#[derive(Clone, Debug)]
pub struct GuildRoster(ArcSignal<Vec<RosterEntry>>);

impl IntoIterator for GuildRoster {
    type Item = RosterEntry;
    type IntoIter = <Vec<RosterEntry> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.get().into_iter()
    }
}

fn user_guild_roster() -> ArcResource<Vec<RosterEntry>, SerdeJson> {
    let planner_context = use_context::<PlannerContext>().unwrap();
    let user = planner_context.user();
    let region = ArcSignal::derive(move || planner_context.region().get());
    let user = ArcMemo::new(move |_| user.ready_or_reloading().flatten());
    ArcResource::new_serde(
        move || {
            (
                user.get().map(|u| u.realm.clone()),
                user.get()
                    .and_then(|u| u.guild)
                    .map(|g| g.to_case(Case::Kebab)),
                region.get(),
            )
        },
        move |(realm, guild, region)| async move {
            if let Some((realm, guild)) = realm.zip(guild) {
                guild_roster(realm.slug.clone(), guild, region)
                    .await
                    .ok()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|gre| RosterEntry {
                        name: gre.name,
                        class: gre.class,
                        realm: Some(realm.clone()),
                    })
                    .collect()
            } else {
                vec![]
            }
        },
    )
}

pub fn provide_guild_roster_context() {
    //let (guild_roster, _) = signal(vec![]);
    //provide_context(GuildRoster(guild_roster.into()))
    let guild_roster = user_guild_roster();
    provide_context(GuildRoster(ArcSignal::derive(move || {
        guild_roster.ready_or_reloading().unwrap_or_default()
    })));
}
