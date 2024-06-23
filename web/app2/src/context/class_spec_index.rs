use std::collections::HashMap;
use std::sync::Arc;

use leptos::*;

use i18n::LocalizedString;

use crate::misc::flatten_ok::FlattenOk;
use crate::serverfns::classes_and_specs;

#[derive(Clone, Debug)]
pub struct ClassSpecIndex {
    classes: Memo<Vec<LocalizedString>>,
    specs: Memo<Vec<LocalizedString>>,
    specs_for_class: Memo<Arc<HashMap<LocalizedString, Vec<LocalizedString>>>>,
}

impl ClassSpecIndex {
    fn new() -> Self {
        let classes_and_specs = Resource::new(|| (), move |_| classes_and_specs());
        let classes = Memo::new(move |_| {
            classes_and_specs
                .get()
                .flatten_ok()
                .map(|cns| cns.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>())
                .unwrap_or_default()
        });
        let specs = Memo::new(move |_| {
            classes_and_specs
                .get()
                .flatten_ok()
                .map(|cns| cns.iter().flat_map(|(_, v)| v.clone()).collect::<Vec<_>>())
                .unwrap_or_default()
        });
        let specs_for_class = Memo::new(move |_| {
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
