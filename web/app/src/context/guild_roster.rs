use crate::components::main::RosterEntry;
use crate::context::{PlannerContext, PlannerRealm, PlannerUser, PlayerClass};
use crate::reactive::resource_ext::ResourceGetExt;
use crate::serverfns::guild_roster;
use auto_battle_net::game_data::playable_class::playable_classes_index::PlayableClassesIndexRequest;
use auto_battle_net::game_data::playable_specialization::playable_specializations_index::PlayableSpecializationsIndexRequest;
use auto_battle_net::profile::guild::guild_roster::GuildRosterRequest;
use auto_battle_net::{BattleNetClientAsync, Region};
use convert_case::{Case, Casing};
use leptos::*;

#[derive(Copy, Clone, Debug)]
pub struct GuildRoster(Signal<Vec<RosterEntry>>);

impl IntoIterator for GuildRoster {
    type Item = RosterEntry;
    type IntoIter = <Vec<RosterEntry> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.get().into_iter()
    }
}

fn user_guild_roster() -> Resource<(Option<PlannerRealm>, Option<String>, Region), Vec<RosterEntry>>
{
    let planner_context = expect_context::<PlannerContext>();
    let user = create_memo(move |_| planner_context.user.get());
    create_resource(
        move || {
            (
                user.get().map(|u| u.realm.clone()),
                user.get()
                    .and_then(|u| u.guild)
                    .map(|g| g.to_case(Case::Kebab)),
                planner_context.region.get(),
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
    //let (guild_roster, _) = create_signal(vec![]);
    //provide_context(GuildRoster(guild_roster.into()))
    let guild_roster = user_guild_roster();
    provide_context(GuildRoster(Signal::derive(move || {
        guild_roster.get().unwrap_or_default()
    })));
}
