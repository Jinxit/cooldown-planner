use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use i18n::{Locale, Region};
use planner::PlannerRealm;

use crate::{misc::flatten_ok::FlattenOk, serverfns::current_main_character};
use crate::serverfns::PlannerMainCharacter;

#[derive(Debug, Clone, Copy)]
pub struct UserContext {
    pub region: AsyncDerived<Region>,
    pub locale: AsyncDerived<Locale>,
    pub main_character: AsyncDerived<Option<PlannerMainCharacter>>,
}

pub fn provide_user_context() {
    let mc = Resource::new(
        || (),
        move |_| async move {
            let mc = current_main_character().await;
            if let Err(e) = &mc {
                error!("Failed to fetch main character: {e:?}");
            }
            mc
        },
    );

    let locale = {
        AsyncDerived::new(move || async move {
            mc.await
                .map(|mc| mc.locale)
                .unwrap_or(Locale::EnglishUnitedStates)
        })
    };

    let region = {
        AsyncDerived::new(
            move || async move { mc.await.map(|mc| mc.region).unwrap_or(Region::Europe) },
        )
    };

    let main_character = {
        AsyncDerived::new(move || async move {
            mc.await
                .map(|mc| PlannerMainCharacter {
                    name: mc.name.clone(),
                    realm: mc.realm.clone(),
                    guild: mc.guild.clone(),
                })
                .ok()
        })
    };

    provide_context(UserContext {
        region,
        locale,
        main_character,
    });
}
