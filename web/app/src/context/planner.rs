use crate::{misc::flatten_ok::FlattenOk, serverfns::current_main_character, reactive::resource_ext::ResourceAndThenExt};
use auto_battle_net::{Locale, LocalizedString, Region};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlannerRealm {
    pub name: LocalizedString,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PlannerUser {
    pub name: String,
    pub realm: PlannerRealm,
    pub guild: Option<String>,
}

// TODO: make this a Resource
#[derive(Debug, Clone)]
pub struct PlannerContext {
    pub region: Signal<Region>,
    pub locale: Signal<Locale>,
    pub user: Signal<Option<PlannerUser>>,
}

pub fn provide_planner_context() {
    let main_character = create_resource(
        || (),
        move |_| async move { current_main_character().await },
    );

    let locale = Signal::derive(move || {
        main_character
            .and_then(|mc| mc.locale)
            .flatten_ok()
            .unwrap_or(Locale::EnglishUnitedStates)
    });

    let region = Signal::derive(move || {
        main_character
            .and_then(|mc| mc.region)
            .flatten_ok()
            .unwrap_or(Region::Europe)
    });

    let user = Signal::derive(move || {
        main_character
            .and_then(|mc| PlannerUser {
                name: mc.name.clone(),
                realm: mc.realm.clone(),
                guild: mc.guild.clone(),
            })
            .flatten_ok()
    });

    provide_context(PlannerContext {
        region,
        locale,
        user,
    });
}
