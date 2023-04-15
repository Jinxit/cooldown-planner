use fight_domain::{Lookup, LookupKey};
use leptos::*;
use memo::Memoize;
use std::fmt::Debug;
use std::hash::Hash;

pub mod blank_suspense;
pub mod map;
pub mod memo;
pub mod resource_ext;

#[component]
pub fn ForEach<IF, I, T, EF, N>(each: IF, children: EF) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T>,
    EF: Fn(T) -> N + 'static,
    N: IntoView + 'static,
    T: Clone + Eq + Hash + 'static,
{
    view! {
        <For
            each
            key=move |value| value.clone()
            view=move |value| { children(value) }
        />
    }
}

#[component]
pub fn ForLookup5<T, F>(#[prop(into)] lookup: Signal<Lookup<T>>, children: F) -> impl IntoView
where
    T: Clone + PartialEq + LookupKey + 'static,
    F: Fn(T) -> Fragment + Clone + 'static,
    T::Key: Eq + Hash + 'static,
{
    view! {
        <For
            clone:children
            each=lookup
            key=move |value| value.lookup_key().clone()
            view=move | value| { children( value) }
        />
    }
}

#[component]
pub fn ForLookup6<T, F>(#[prop(into)] lookup: Signal<Lookup<T>>, children: F) -> impl IntoView
where
    T: Clone + PartialEq + Hash + Eq + LookupKey + 'static,
    F: Fn(T) -> Fragment + Clone + 'static,
    T::Key: Eq + Hash + 'static,
{
    view! {
        <For
            clone:children
            each=lookup
            key=move |value| value.clone()
            view=move | value| { children( value) }
        />
    }
}
