use std::convert::Infallible;

use axum::{async_trait, Extension};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::Storage;
use crate::store::{Connection, ConnectionHolder, Store};

#[async_trait]
impl<S> FromRequestParts<S> for Storage
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let extensions = &req.extensions;
        let connection_holder = extensions.get::<ConnectionHolder>().unwrap();
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
