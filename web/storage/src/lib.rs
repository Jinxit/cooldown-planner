use std::fmt::Debug;
use std::future::Future;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use keyable::Keyable;

use crate::store::{Store, StoreKey};

mod keyable;
mod store;

#[cfg(feature = "axum")]
pub mod axum;
#[cfg(feature = "sqlite")]
pub mod sqlite;

#[derive(Clone, Debug)]
pub struct Storage {
    store: Store,
}

impl Storage {
    pub fn new(store: Store) -> Self {
        Self { store }
    }

    pub async fn fetch<K, V, Fetcher, Fut>(&self, key: &K, ttl: Duration, fetcher: Fetcher) -> V
    where
        K: Keyable + 'static,
        V: Serialize + DeserializeOwned + Send + Sync + 'static,
        Fetcher: FnOnce() -> Fut,
        Fut: Future<Output = V>,
    {
        let store_key = StoreKey::new::<K, V>(key);
        match self.store.get(&store_key).await {
            Some(value) => serde_json::from_value(value).unwrap(),
            None => {
                let value = fetcher().await;
                self.store
                    .put(
                        &store_key,
                        serde_json::to_value(&value).unwrap(),
                        Some(&ttl),
                    )
                    .await;
                value
            }
        }
    }

    pub async fn try_fetch<K, V, E, Fetcher, Fut>(
        &self,
        key: &K,
        ttl: Duration,
        fetcher: Fetcher,
    ) -> Result<V, E>
    where
        K: Keyable + 'static,
        V: Serialize + DeserializeOwned + Send + Sync + 'static,
        Fetcher: FnOnce() -> Fut,
        Fut: Future<Output = Result<V, E>>,
    {
        let store_key = StoreKey::new::<K, V>(key);
        match self.store.get(&store_key).await {
            Some(value) => Ok(serde_json::from_value(value).unwrap()),
            None => {
                let result = fetcher().await;
                if let Ok(value) = &result {
                    self.store
                        .put(&store_key, serde_json::to_value(value).unwrap(), Some(&ttl))
                        .await;
                }
                result
            }
        }
    }

    pub async fn put<K, V>(&self, key: &K, ttl: Duration, value: &V)
    where
        K: Keyable + 'static,
        V: Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        let store_key = StoreKey::new::<K, V>(key);
        self.store
            .put(&store_key, serde_json::to_value(value).unwrap(), Some(&ttl))
            .await;
    }

    pub async fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: Keyable + 'static,
        V: Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        let store_key = StoreKey::new::<K, V>(key);
        self.store
            .get(&store_key)
            .await
            .and_then(|v| serde_json::from_value(v).ok())
    }

    pub async fn clear<K, V>(&self, key: &K)
    where
        K: Keyable + 'static,
        V: Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        let store_key = StoreKey::new::<K, V>(key);
        self.store.clear(&store_key).await
    }
}

pub(crate) fn get_system_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        // This is safe, it only fails if the current system time is before the
        // UNIX_EPOCH. So it will only fail if a time traveler from 1970 is
        // using it.
        .unwrap()
        .as_secs()
}
