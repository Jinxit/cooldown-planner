use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use auto_battle_net::{Locale, LocalizedString, Region};

use crate::{
    misc::flatten_ok::FlattenOk, reactive::async_ext::ReadyOrReloading,
    serverfns::current_main_character,
};

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
    region: ArcSignal<Region>,
    locale: ArcSignal<Locale>,
    user: ArcAsyncDerived<Option<PlannerUser>>,
}

impl PlannerContext {
    pub fn region(&self) -> Signal<Region> {
        let region = self.region.clone();
        Signal::derive(move || region.get())
    }

    pub fn locale(&self) -> Signal<Locale> {
        let locale = self.locale.clone();
        Signal::derive(move || locale.get())
    }

    pub fn user(&self) -> AsyncDerived<Option<PlannerUser>> {
        let user = self.user.clone();
        AsyncDerived::new(move || {
            let user = user.clone();
            async move { user.await }
        })
    }
}

pub fn provide_planner_context() {
    let main_character = ArcResource::new_serde(
        || (),
        move |_| async move { current_main_character().await },
    );

    let locale = {
        let main_character = main_character.clone();
        ArcSignal::derive(move || {
            main_character
                .ready_or_reloading()
                .flatten_ok()
                .map(|mc| mc.locale)
                .unwrap_or(Locale::EnglishUnitedStates)
        })
    };

    let region = {
        let main_character = main_character.clone();
        ArcSignal::derive(move || {
            main_character
                .ready_or_reloading()
                .flatten_ok()
                .map(|mc| mc.region)
                .unwrap_or(Region::Europe)
        })
    };

    let user = {
        let main_character = main_character.clone();
        ArcAsyncDerived::new(move || {
            let main_character = main_character.clone();
            async move {
                main_character
                    .await
                    .map(|mc| PlannerUser {
                        name: mc.name.clone(),
                        realm: mc.realm.clone(),
                        guild: mc.guild.clone(),
                    })
                    .ok()
            }
        })
    };

    provide_context(PlannerContext {
        region,
        locale,
        user,
    });
}
