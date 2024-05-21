use std::fmt::Debug;

use leptos::prelude::*;

pub trait ReadyOrReloading<T> {
    fn ready_or_reloading(self) -> Option<T>;
}

impl<T: Clone + Send + Sync + 'static> ReadyOrReloading<T> for AsyncDerived<T> {
    fn ready_or_reloading(self) -> Option<T> {
        match self.get() {
            AsyncState::Loading => None,
            AsyncState::Complete(value) => Some(value),
            AsyncState::Reloading(value) => Some(value),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> ReadyOrReloading<T> for &ArcAsyncDerived<T> {
    fn ready_or_reloading(self) -> Option<T> {
        match self.get() {
            AsyncState::Loading => None,
            AsyncState::Complete(value) => Some(value),
            AsyncState::Reloading(value) => Some(value),
        }
    }
}

impl<T: Clone + Send + Sync + 'static + Debug, Ser: Send + Sync + 'static> ReadyOrReloading<T>
    for Resource<T, Ser>
{
    fn ready_or_reloading(self) -> Option<T> {
        let get = self.get();
        match get {
            AsyncState::Loading => None,
            AsyncState::Complete(value) => Some(value),
            AsyncState::Reloading(value) => Some(value),
        }
    }
}
