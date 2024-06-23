use std::fmt::Debug;
use std::future::Future;
use leptos::prelude::*;
use leptos::prelude::graph::ReactiveNode;
use leptos::prelude::serializers::{SerdeJson, SerializableData};

pub fn local_resource<T, S, Fut>(
    source: impl Fn() -> S + Send + Sync + 'static,
    fetcher: impl Fn(S) -> Fut + Clone + Send + Sync + 'static,
) -> Resource<Option<T>, SerdeJson>
    where
        T: SerializableData<SerdeJson> + Debug + Send + Sync + 'static,
        Option<T>: SerializableData<SerdeJson>,
        S: Clone + PartialEq + Send + Sync + 'static,
        Fut: Future<Output = T> + Send + 'static,
        T::SerErr: Debug,
        <Option<T> as SerializableData<SerdeJson>>::SerErr: Debug,
        T::DeErr: Debug,
        <Option<T> as SerializableData<SerdeJson>>::DeErr: Debug,
{
        let resource: Resource<Option<T>, SerdeJson> =
        Resource::new(source, move |s| {
            let fetcher = fetcher.clone();
            async move {
                if cfg!(feature = "hydrate") {
                    let value = fetcher(s).await;
                    Some(value)
                } else {
                    None
                }
            }
        });
    Effect::new(move |_| {
        resource.mark_dirty();
    });
    resource
}