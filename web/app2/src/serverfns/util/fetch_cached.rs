use super::get_storage;
use futures_util::Future;
use leptos::*;
use leptos_axum::extract;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::time::Duration;
use storage::Keyable;

pub async fn fetch_cached<K, V, Fetcher, Fut>(key: &K, ttl: Duration, fetcher: Fetcher) -> V
where
    K: Keyable + Send + 'static,
    V: Serialize + DeserializeOwned + Send + Sync + Clone + 'static,
    Fetcher: FnOnce() -> Fut,
    Fut: Future<Output = V>,
{
    let storage = get_storage().await;
    storage.fetch(key, ttl, fetcher).await
}

pub async fn storage.try_fetch<K, V, E, Fetcher, Fut>(
    key: &K,
    ttl: Duration,
    fetcher: Fetcher,
) -> Result<V, E>
where
    K: Keyable + Send + 'static,
    V: Serialize + DeserializeOwned + Send + Sync + Clone + 'static,
    Fetcher: FnOnce() -> Fut,
    Fut: Future<Output = Result<V, E>> + Send,
{
    let storage = get_storage().await;
    storage.try_fetch(key, ttl, fetcher).await
}
