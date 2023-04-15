#![allow(clippy::arc_with_non_send_sync)]

use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{headers, TypedHeader};
use cookie::{CookieBuilder, SameSite};
use core::convert::TryFrom;
use http::{HeaderValue, Request};
use std::env::var;

use pasetors::claims::{Claims, ClaimsValidationRules};

use pasetors::keys::SymmetricKey;

use pasetors::token::UntrustedToken;
use pasetors::{local, version4::V4, Local};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use tracing::warn;

#[derive(Debug)]
pub struct Session<T> {
    data: Arc<RwLock<T>>,
    original_data: Arc<RwLock<T>>,
}

impl<T> Clone for Session<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            original_data: self.original_data.clone(),
        }
    }
}

impl<T> Session<T>
where
    T: Default,
{
    fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(T::default())),
            original_data: Arc::new(RwLock::new(T::default())),
        }
    }
}

impl<T> Session<T>
where
    T: Eq,
{
    fn was_modified(&self) -> bool {
        *self.data.read().unwrap() != *self.original_data.read().unwrap()
    }
}

impl<T> Session<T> {
    fn ref_count(&self) -> usize {
        Arc::strong_count(&self.data)
    }

    pub fn data(&self) -> impl Deref<Target = T> + '_ {
        let lock = self.data.read();
        lock.unwrap()
    }

    pub fn data_mut(&self) -> impl DerefMut<Target = T> + '_ {
        let lock = self.data.write();
        lock.unwrap()
    }
}

pub async fn session_middleware<T, B>(
    TypedHeader(cookie): TypedHeader<headers::Cookie>,
    mut request: Request<B>,
    next: Next<B>,
) -> impl IntoResponse
where
    T: Debug + Default + Eq + Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    let session_secret = var("PASETO_SESSION_SECRET").unwrap();
    let sk = SymmetricKey::<V4>::try_from(session_secret.as_str()).unwrap();

    let session = {
        let mut session = Session::new();
        if let Some(token) = cookie.get("paseto") {
            let validation_rules = ClaimsValidationRules::new();
            let untrusted_token = UntrustedToken::<Local, V4>::try_from(token);

            if let Ok(untrusted_token) = untrusted_token {
                let trusted_token = local::decrypt(
                    &sk,
                    &untrusted_token,
                    &validation_rules,
                    None,
                    Some(b"implicit assertion"),
                );

                if let Ok(trusted_token) = trusted_token {
                    let data = serde_json::from_value::<T>(
                        trusted_token
                            .payload_claims()
                            .unwrap()
                            .get_claim("data")
                            .unwrap()
                            .clone(),
                    );

                    if let Ok(data) = data {
                        session = Session {
                            data: Arc::new(RwLock::new(data.clone())),
                            original_data: Arc::new(RwLock::new(data)),
                        }
                    }
                }
            }
        }
        session
    };

    request.extensions_mut().insert(session.clone());

    let mut response = next.run(request).await;

    if session.was_modified() {
        let ref_count = session.ref_count();
        if ref_count > 1 {
            warn!(
                "Session was modified after headers were sent - use `create_blocking_resource`!\nNumber of owners: {ref_count}\n"
            );
        }
        let mut claims = Claims::new().unwrap();
        claims
            .add_additional(
                "data",
                serde_json::to_value(session.data.read().unwrap().clone()).unwrap(),
            )
            .unwrap();
        let token = local::encrypt(&sk, &claims, None, Some(b"implicit assertion")).unwrap();
        response.headers_mut().insert(
            http::header::SET_COOKIE,
            HeaderValue::from_str(
                &CookieBuilder::new("paseto", &token)
                    .http_only(true)
                    .same_site(SameSite::Lax)
                    .secure(true)
                    .path("/")
                    .expires(Some(
                        (SystemTime::now() + Duration::from_secs(24 * 60 * 60)).into(),
                    ))
                    .finish()
                    .to_string(),
            )
            .unwrap(),
        );
    }

    response
}
