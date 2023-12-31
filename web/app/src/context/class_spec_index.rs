use crate::context::PlannerContext;
use crate::misc::flatten_ok::FlattenOk;
use crate::reactive::resource_ext::{ResourceAndThenExt, ResourceGetExt};
use crate::serverfns::classes_and_specs;
use auto_battle_net::game_data::playable_class::playable_class::PlayableClassRequest;
use auto_battle_net::game_data::playable_class::playable_classes_index::{
    Classes, PlayableClassesIndexRequest,
};
use auto_battle_net::game_data::playable_specialization::playable_specialization::PlayableSpecializationRequest;
use auto_battle_net::game_data::playable_specialization::playable_specializations_index::{
    CharacterSpecializations, PlayableSpecializationsIndexRequest,
};
use auto_battle_net::{BattleNetClientAsync, LocalizedString};
use futures_util::future::try_join_all;
use leptos::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ClassSpecIndex {
    classes: Memo<Vec<LocalizedString>>,
    specs: Memo<Vec<LocalizedString>>,
    specs_for_class: Memo<Arc<HashMap<LocalizedString, Vec<LocalizedString>>>>,
}

impl ClassSpecIndex {
    fn new() -> Self {
        let classes_and_specs = create_resource(|| (), move |_| classes_and_specs());
        let classes = create_memo(move |_| {
            classes_and_specs
                .and_then(|cns| cns.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>())
                .flatten_ok()
                .unwrap_or_default()
        });
        let specs = create_memo(move |_| {
            classes_and_specs
                .and_then(|cns| cns.iter().flat_map(|(_, v)| v.clone()).collect::<Vec<_>>())
                .flatten_ok()
                .unwrap_or_default()
        });
        let specs_for_class = create_memo(move |_| {
            Arc::new(
                classes_and_specs
                    .get()
                    .flatten_ok()
                    .unwrap_or_default()
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            )
        });

        Self {
            classes,
            specs,
            specs_for_class,
        }
    }

    pub fn specs_for_class(self, class: LocalizedString) -> Signal<Option<Vec<LocalizedString>>> {
        Signal::derive(move || {
            self.specs_for_class
                .with(|specs_for_class| specs_for_class.get(&class).cloned())
        })
    }

    pub fn classes(self) -> Signal<Vec<LocalizedString>> {
        self.classes.into()
    }

    pub fn specs(self) -> Signal<Vec<LocalizedString>> {
        self.specs.into()
    }
}

pub fn provide_class_spec_index_context() {
    provide_context(ClassSpecIndex::new());
}
