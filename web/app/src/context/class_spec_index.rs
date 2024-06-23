use std::collections::HashMap;
use std::sync::Arc;

use leptos::prelude::*;

use i18n::LocalizedString;

use crate::misc::flatten_ok::FlattenOk;
use crate::serverfns::classes_and_specs;

#[derive(Copy, Clone)]
pub struct ClassSpecIndex {
    classes: AsyncDerived<Vec<LocalizedString>>,
    specs: AsyncDerived<Vec<LocalizedString>>,
    specs_for_class: AsyncDerived<Arc<HashMap<LocalizedString, Vec<LocalizedString>>>>,
}

impl ClassSpecIndex {
    fn new() -> Self {
        let classes_and_specs = Resource::new(
            || (),
            move |_| async move {
                let classes_and_specs = classes_and_specs().await;
                classes_and_specs.unwrap_or_else(|e| {
                    tracing::error!("{:?}", e);
                    vec![]
                })
            },
        );
        let classes = AsyncDerived::new({
            move || {
                async move {
                    classes_and_specs
                        .await
                        .iter()
                        .map(|(c, _)| c.clone())
                        .collect::<Vec<_>>()
                }
            }
        });
        let specs = AsyncDerived::new({
            move || {
                async move {
                    classes_and_specs
                        .await
                        .iter()
                        .flat_map(|(_, s)| s.iter().cloned())
                        .collect::<Vec<_>>()
                }
            }
        });
        let specs_for_class = AsyncDerived::new({
            move || {
                async move {
                    Arc::new(
                        classes_and_specs
                            .await
                            .into_iter()
                            .collect::<HashMap<_, _>>(),
                    )
                }
            }
        });

        Self {
            classes,
            specs,
            specs_for_class,
        }
    }

    pub fn specs_for_class(&self, class: LocalizedString) -> AsyncDerived<Vec<LocalizedString>> {
        let specs_for_class = self.specs_for_class.clone();
        AsyncDerived::new(move || {
            let class = class.clone();
            let specs_for_class = specs_for_class.clone();
            async move {
                specs_for_class
                    .await
                    .get(&class)
                    .cloned()
                    .unwrap_or_else(|| {
                        tracing::error!("No specs found for class {class:?}");
                        vec![]
                    })
            }
        })
    }

    pub fn classes(&self) -> AsyncDerived<Vec<LocalizedString>> {
        self.classes.clone().into()
    }

    pub fn specs(&self) -> AsyncDerived<Vec<LocalizedString>> {
        self.specs.clone().into()
    }
}

pub fn provide_class_spec_index_context() {
    provide_context(ClassSpecIndex::new());
}

pub fn use_class_spec_index() -> ClassSpecIndex {
    use_context::<ClassSpecIndex>().unwrap()
}
