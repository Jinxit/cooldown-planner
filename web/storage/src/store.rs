use async_trait::async_trait;
use serde_json::Value;
use std::any::TypeId;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

pub struct Store<T> {
    connection: Arc<dyn ConnectionDyn>,
    _phantom: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct StoreKey {
    pub(crate) key: String,
    pub(crate) key_type: TypeId,
    pub(crate) value_type: TypeId,
}

impl<T> Store<T> {
    pub fn new(connection_holder: &ConnectionHolder) -> Self {
        Self {
            connection: connection_holder.connection.clone(),
            _phantom: PhantomData,
        }
    }

    pub async fn get(&self, key: &StoreKey) -> Option<Value> {
        let fut = self.connection.get(key);
        fut.await
    }

    pub async fn put(&self, key: &StoreKey, value: Value, ttl: Option<&Duration>) {
        self.connection.put(key, value, ttl).await
    }

    pub async fn clear(&self, key: &StoreKey) {
        self.connection.delete(key).await
    }
}

impl<T> Clone for Store<T> {
    fn clone(&self) -> Self {
        Self {
            connection: self.connection.clone(),
            _phantom: PhantomData,
        }
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
