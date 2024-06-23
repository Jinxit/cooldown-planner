use std::any::TypeId;
use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use key_mutex::tokio::KeyRwLock;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::Keyable;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct StoreKey {
    pub(crate) key: String,
    pub(crate) key_type: TypeId,
    pub(crate) value_type: TypeId,
    _phantom: PhantomData<()>,
}

impl StoreKey {
    pub fn new<K, V>(key: &impl Keyable) -> Self
    where
        K: Keyable + 'static,
        V: Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        StoreKey {
            key: key.to_key(),
            key_type: TypeId::of::<K>(),
            value_type: TypeId::of::<V>(),
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct Store {
    connection: Arc<dyn ConnectionDyn>,
    locks: KeyRwLock<StoreKey, ()>,
}

impl Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store").finish()
    }
}

impl Store {
    pub fn new(connection_holder: &ConnectionHolder) -> Self {
        Self {
            connection: connection_holder.connection.clone(),
            locks: KeyRwLock::new(),
        }
    }

    pub async fn get(&self, key: &StoreKey) -> Option<Value> {
        let _lock = self.locks.read(key.clone()).await;
        let fut = self.connection.get(key);
        fut.await
    }

    pub async fn put(&self, key: &StoreKey, value: Value, ttl: Option<&Duration>) {
        let _lock = self.locks.write(key.clone()).await;
        self.connection.put(key, value, ttl).await
    }

    pub async fn clear(&self, key: &StoreKey) {
        let _lock = self.locks.write(key.clone()).await;
        self.connection.delete(key).await
    }
}

#[derive(Clone)]
pub struct ConnectionHolder {
    connection: Arc<dyn ConnectionDyn>,
}

impl ConnectionHolder {
    pub fn new(connection: impl Connection + 'static) -> Self {
        Self {
            connection: Arc::new(connection),
        }
    }
}

#[async_trait]
pub trait Connection: Send + Sync {
    async fn get(&self, key: &StoreKey) -> Option<Value>;
    async fn put(&self, key: &StoreKey, value: Value, ttl: Option<Duration>);
    async fn delete(&self, key: &StoreKey);
}

pub(crate) trait ConnectionDyn: Send + Sync {
    fn get<'a>(
        &'a self,
        key: &'a StoreKey,
    ) -> Pin<Box<dyn Future<Output = Option<Value>> + Send + '_>>;
    fn put<'a>(
        &'a self,
        key: &'a StoreKey,
        value: Value,
        ttl: Option<&Duration>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
    fn delete<'a>(&'a self, key: &'a StoreKey) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

impl<T: Connection> ConnectionDyn for T {
    fn get<'a>(
        &'a self,
        key: &'a StoreKey,
    ) -> Pin<Box<dyn Future<Output = Option<Value>> + Send + '_>> {
        Box::pin(<Self as Connection>::get(self, key))
    }

    fn put<'a>(
        &'a self,
        key: &'a StoreKey,
        value: Value,
        ttl: Option<&Duration>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let ttl = ttl.cloned();
        Box::pin(<Self as Connection>::put(self, key, value, ttl))
    }

    fn delete<'a>(&'a self, key: &'a StoreKey) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(<Self as Connection>::delete(self, key))
    }
}
