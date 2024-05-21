use fight_domain::{Lookup, LookupKey};
use leptos::prelude::*;
use memo::Memoize;
use std::fmt::Debug;
use std::hash::Hash;

pub mod async_ext;
pub mod memo;

#[component]
pub fn ForEach<IF, I, T, EF, N>(each: IF, children: EF) -> impl IntoView
where
    IF: Fn() -> I + Send + 'static,
    I: IntoIterator<Item = T> + Send + 'static,
    EF: Fn(T) -> N + Clone + Send + 'static,
    N: IntoView + Send + 'static,
    T: Clone + Eq + Hash + Send + 'static,
{
    view! { <For each key=move |value| value.clone() children=move |value| { children(value) }/> }
}

#[component]
pub fn ForLookup5<T, F, N>(#[prop(into)] lookup: Signal<Lookup<T>>, children: F) -> impl IntoView
where
    T: Clone + PartialEq + LookupKey + Send + Sync + 'static,
    F: Fn(T) -> N + Clone + Send + 'static,
    N: IntoView + Send + 'static,
    T::Key: Eq + Hash + Send + Sync + 'static,
{
    view! {
        <For
            each=lookup
            key=move |value| value.lookup_key().clone()
            children=move |value| { children(value) }
        />
    }
}

#[component]
pub fn ForLookup6<T, F, N>(#[prop(into)] lookup: Signal<Lookup<T>>, children: F) -> impl IntoView
where
    T: Clone + PartialEq + Hash + Eq + LookupKey + Send + Sync + 'static,
    F: Fn(T) -> N + Clone + Send + 'static,
    N: IntoView + Send + 'static,
    T::Key: Eq + Hash + Send + Sync + 'static,
{
    view! {
        <For each=lookup key=move |value| value.clone() children=move |value| { children(value) }/>
    }
}
