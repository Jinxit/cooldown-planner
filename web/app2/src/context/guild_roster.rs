use convert_case::{Case, Casing};
use leptos::*;
use i18n::Region;

use crate::components::main::RosterEntry;
use crate::context::{PlannerContext, PlannerRealm};
use crate::serverfns::guild_roster;

#[derive(Clone, Debug)]
pub struct GuildRoster(Signal<Vec<RosterEntry>>);

impl IntoIterator for GuildRoster {
    type Item = RosterEntry;
    type IntoIter = <Vec<RosterEntry> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.get().into_iter()
    }
}

fn user_guild_roster() -> Resource<(Option<PlannerRealm>, Option<String>, Region), Vec<RosterEntry>> {
    let planner_context = use_context::<PlannerContext>().unwrap();
    let user = planner_context.user.clone();
    let region = planner_context.region.clone();
    let user = Memo::new(move |_| user.get());
    Resource::new(
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
    provide_context(GuildRoster(Signal::derive(move || {
        guild_roster.get().unwrap_or_default()
    })));
}
