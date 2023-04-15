use crate::store::{Connection, ConnectionHolder, Store};
use crate::{Keyable, Storage};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::Extension;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::Infallible;

#[axum::async_trait]
impl<K, V, S> FromRequestParts<S> for Storage<K, V>
where
    K: Keyable + 'static,
    V: Serialize + DeserializeOwned + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let connection_holder = req.extensions.get::<ConnectionHolder>().unwrap();
        let store = Store::new(connection_holder);
        let storage = Storage::new(store);

        Ok(storage)
    }
}

pub trait StoreExt {
    fn with_store(self, connection: impl Connection + 'static) -> Self;
}

impl<S> StoreExt for axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_store(self, connection: impl Connection + 'static) -> Self {
        self.layer(Extension(ConnectionHolder::new(connection)))
    }
}
