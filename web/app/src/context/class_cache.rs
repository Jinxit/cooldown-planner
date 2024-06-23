use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex, RwLock};

use convert_case::{Case, Casing};
use leptos::prelude::*;
use leptos::prelude::serializers::SerdeJson;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

use auto_battle_net::BattleNetClientAsync;
use i18n::{Locale, LocalizedString};

use crate::reactive::memo::Memoize;
use crate::serverfns::{character_summary, PlayerClass};

use super::UserContext;

#[derive(Clone)]
pub struct ClassCache {
    cache: Arc<Mutex<ClassSignalMap>>,
}

type ClassSignalMap = HashMap<(String, String), ArcResource<PlayerClass>>;

impl ClassCache {
    fn new() -> Self {
        Self {
            cache: Default::default(),
        }
    }

    pub fn get(&self, character_name: &str, realm_name: &LocalizedString) -> AsyncDerived<PlayerClass> {
        let mut cache = self.cache.lock().unwrap();
        let entry = cache.entry((
            character_name.to_string(),
            realm_name.get(Locale::EnglishUnitedStates).to_string(),
        ));
        let character_name = character_name.to_case(Case::Kebab);
        let realm_slug = realm_name
            .get(Locale::EnglishUnitedStates)
            .to_case(Case::Kebab);
        // TODO: region needs to be part of the key too
        let region = use_context::<UserContext>().unwrap().region.clone();
        let resource = entry
            .or_insert_with(move || {
                ArcResource::new(|| (), move |_| {
                    let character_name = character_name.clone();
                    let realm_slug = realm_slug.clone();
                    async move {
                        let summary =
                            character_summary(character_name, realm_slug, region.await).await;
                        match summary {
                            Ok(resp) => PlayerClass::Known(resp.character_class.name),
                            Err(_) => PlayerClass::Missing,
                        }
                    }
                })
            });
        let derived: ArcAsyncDerived<_> = (*(resource.clone())).clone();
        derived.into()
    }
}

pub fn provide_class_cache_context() {
    provide_context(ClassCache::new());
}
