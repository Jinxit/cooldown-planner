use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use convert_case::{Case, Casing};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

use auto_battle_net::{BattleNetClientAsync, Locale, LocalizedString};

use crate::reactive::memo::Memoize;
use crate::serverfns::character_summary;

use super::PlannerContext;

#[derive(Clone, Debug)]
pub struct ClassCache {
    cache: Arc<Mutex<ClassSignalMap>>,
}
// TODO: switch to Resource since that's what it actually is
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
    pub fn new() -> Self {
        Self {
            cache: Default::default(),
        }
    }

    pub fn get(&self, character_name: &str, realm_name: &LocalizedString) -> Signal<PlayerClass> {
        let mut borrow = self.cache.lock().unwrap();
        let entry = borrow.entry((
            character_name.to_string(),
            realm_name.get(Locale::EnglishUnitedStates).to_string(),
        ));
        let character_name = character_name.to_case(Case::Kebab);
        let realm_slug = realm_name
            .get(Locale::EnglishUnitedStates)
            .to_case(Case::Kebab);
        // TODO: region needs to be part of the key too
        let region =
            Signal::derive(move || use_context::<PlannerContext>().unwrap().region().get());
        *entry.or_insert_with(move || {
            let (class, set_class) = arc_signal(PlayerClass::Unknown);
            Effect::new(move |_| {
                let character_name = character_name.clone();
                let realm_slug = realm_slug.clone();
                let set_class = set_class.clone();
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
    provide_context(ClassCache::new());
}
