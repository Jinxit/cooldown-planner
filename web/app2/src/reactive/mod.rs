use fight_domain::{Lookup, LookupKey};
use leptos::*;
use memo::Memoize;
use std::fmt::Debug;
use std::hash::Hash;

pub mod memo;

#[component]
pub fn ForEach<IF, I, T, EF, N>(each: IF, children: EF) -> impl IntoView
where
    IF: Fn() -> I  + 'static,
    I: IntoIterator<Item = T> + 'static,
    EF: Fn(T) -> N + 'static,
    N: IntoView + 'static,
    T: Clone + Eq + Hash + 'static,
{
    view! { <For each key=move |value| value.clone() children=move |value| { children(value) }/> }
}

#[component]
pub fn ForLookup5<T, F, N>(#[prop(into)] lookup: Signal<Lookup<T>>, children: F) -> impl IntoView
where
    T: Clone + PartialEq + LookupKey + 'static,
    F: Fn(T) -> N + Clone + 'static,
    N: IntoView + 'static,
    T::Key: Eq + Hash + 'static,
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
    F: Fn(T) -> N + Clone + 'static,
    N: IntoView + 'static,
    T::Key: Eq + Hash + 'static,
{
    view! {
        <For each=lookup key=move |value| value.clone() children=move |value| { children(value) }/>
    }
}
