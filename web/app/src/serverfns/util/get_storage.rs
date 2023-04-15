use leptos::*;
use leptos_axum::extract;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;
use std::time::Duration;
use storage::Keyable;

pub async fn get_storage<
    K: Keyable + Send + 'static,
    V: Serialize + DeserializeOwned + Send + Sync + Clone + 'static,
>() -> storage::Storage<K, V> {
    extract(move |storage: storage::Storage<K, V>| async move { storage })
        .await
        .unwrap()
}
