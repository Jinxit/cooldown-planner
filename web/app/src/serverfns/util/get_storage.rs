use leptos::prelude::*;
use leptos_axum::extract;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;
use std::time::Duration;
use storage::Keyable;

pub async fn get_storage() -> Result<storage::Storage, ServerFnError> {
    extract()
        .await
}
