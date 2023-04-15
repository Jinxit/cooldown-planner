use crate::reactive::memo::Memoize;
use crate::serverfns::character_summary;
use auto_battle_net::profile::character_profile::character_profile_summary::CharacterProfileSummaryRequest;
use auto_battle_net::{BattleNetClientAsync, Locale, LocalizedString};
use convert_case::{Case, Casing};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::PlannerContext;

#[derive(Clone, Debug)]
pub struct ClassCache {
    cache: Rc<RefCell<ClassSignalMap>>,
}
type ClassSignalMap = HashMap<(String, String), Signal<PlayerClass>>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum PlayerClass {
    Known(LocalizedString),
    Unknown,
    Missing,
}

impl PlayerClass {
    pub fn opt(&self) -> Option<&LocalizedString> {
        match self {
            PlayerClass::Known(name) => Some(name),
            PlayerClass::Unknown => None,
            PlayerClass::Missing => None,
        }
    }
}

impl ClassCache {
    pub fn new() -> Signal<Self> {
        Signal::derive(move || Self {
            cache: Default::default(),
        })
    }

    pub fn get(&self, character_name: &str, realm_name: &LocalizedString) -> Signal<PlayerClass> {
        let mut borrow = self.cache.borrow_mut();
        let entry = borrow.entry((
            character_name.to_string(),
            realm_name.get(Locale::EnglishUnitedStates).to_string(),
        ));
        let character_name = character_name.to_case(Case::Kebab);
        let realm_slug = realm_name
            .get(Locale::EnglishUnitedStates)
            .to_case(Case::Kebab);
        let region = expect_context::<PlannerContext>().region;
        *entry.or_insert_with(move || {
            let (class, set_class) = create_signal(PlayerClass::Unknown);
            create_effect(move |_| {
                let character_name = character_name.clone();
                let realm_slug = realm_slug.clone();
                spawn_local(async move {
                    let summary = character_summary(character_name, realm_slug, region.get()).await;
                    set_class(match summary {
                        Ok(resp) => PlayerClass::Known(resp.character_class.name),
                        Err(_) => PlayerClass::Missing,
                    })
                })
            });

            class.memo().into()
        })
    }
}

pub fn provide_class_cache_context() {
    let owner = Owner::current();
    create_effect(move |_| {
        with_owner(owner.unwrap(), move || {
            provide_context(ClassCache::new().get());
        })
    });
}
