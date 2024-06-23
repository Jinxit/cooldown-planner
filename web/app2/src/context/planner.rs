use leptos::*;
use serde::{Deserialize, Serialize};

use i18n::{Locale, LocalizedString, Region};

use crate::{
    misc::flatten_ok::FlattenOk,
    serverfns::current_main_character,
};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PlannerUser {
    pub name: String,
    pub realm: PlannerRealm,
    pub guild: Option<String>,
}

// TODO: make this a Resource
#[derive(Debug, Clone, Copy)]
pub struct PlannerContext {
    pub region: Signal<Region>,
    pub locale: Signal<Locale>,
    pub user: Signal<Option<PlannerUser>>,
}

pub fn provide_planner_context() {
    let main_character = create_local_resource(
        || (),
        move |_| async move { current_main_character().await },
    );

    let locale = {
        let main_character = main_character.clone();
        Signal::derive(move || {
            main_character
                .get()
                .flatten_ok()
                .map(|mc| mc.locale)
                .unwrap_or(Locale::EnglishUnitedStates)
        })
    };

    let region = {
        let main_character = main_character.clone();
        Signal::derive(move || {
            main_character
                .get()
                .flatten_ok()
                .map(|mc| mc.region)
                .unwrap_or(Region::Europe)
        })
    };

    let user = {
        let main_character = main_character.clone();
        Signal::derive(move || {
                main_character
                    .get()
                    .flatten_ok()
                    .map(|mc| PlannerUser {
                        name: mc.name.clone(),
                        realm: mc.realm.clone(),
                        guild: mc.guild.clone(),
                    })
        })
    };

    provide_context(PlannerContext {
        region,
        locale,
        user,
    });
}
