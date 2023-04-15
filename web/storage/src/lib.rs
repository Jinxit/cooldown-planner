mod keyable;
mod store;

pub use keyable::Keyable;

#[cfg(feature = "axum")]
pub mod axum;
#[cfg(feature = "sqlite")]
pub mod sqlite;

use crate::store::{Store, StoreKey};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::any::{type_name, TypeId};
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::marker::PhantomData;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Storage<K, V> {
    store: Store<V>,
    _phantom: PhantomData<K>,
}

impl<K, V> Clone for Storage<K, V> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<K, V> Debug for Storage<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Storage<{}, {}>", type_name::<K>(), type_name::<V>())
    }
}

impl<K, V> Storage<K, V>
where
    K: Keyable + 'static,
    V: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub fn new(store: Store<V>) -> Self {
        Self {
            store,
            _phantom: PhantomData,
        }
    }

    pub async fn fetch<Fetcher, Fut>(&self, key: &K, ttl: Duration, fetcher: Fetcher) -> V
    where
        Fetcher: FnOnce() -> Fut,
        Fut: Future<Output = V>,
    {
        let store_key = StoreKey {
            key: key.to_key(),
            key_type: TypeId::of::<K>(),
            value_type: TypeId::of::<V>(),
        };
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

    pub async fn try_fetch<E, Fetcher, Fut>(
        &self,
        key: &K,
        ttl: Duration,
        fetcher: Fetcher,
    ) -> Result<V, E>
    where
        Fetcher: FnOnce() -> Fut,
        Fut: Future<Output = Result<V, E>>,
    {
        let store_key = StoreKey {
            key: key.to_key(),
            key_type: TypeId::of::<K>(),
            value_type: TypeId::of::<V>(),
        };
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

    pub async fn put(&self, key: &K, ttl: Duration, value: &V) {
        let store_key = StoreKey {
            key: key.to_key(),
            key_type: TypeId::of::<K>(),
            value_type: TypeId::of::<V>(),
        };
        self.store
            .put(&store_key, serde_json::to_value(value).unwrap(), Some(&ttl))
            .await;
    }

    pub async fn get(&self, key: &str) -> Option<V> {
        let store_key = StoreKey {
            key: key.to_string(),
            key_type: TypeId::of::<K>(),
            value_type: TypeId::of::<V>(),
        };
        self.store
            .get(&store_key)
            .await
            .and_then(|v| serde_json::from_value(v).ok())
    }

    pub async fn clear(&self, key: &str) {
        let store_key = StoreKey {
            key: key.to_string(),
            key_type: TypeId::of::<K>(),
            value_type: TypeId::of::<V>(),
        };
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
