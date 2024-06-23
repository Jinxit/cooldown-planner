#![allow(incomplete_features)]
#![allow(async_fn_in_trait)]
#![allow(clippy::module_inception)]

use ordered_float::NotNan;
use serde_lite::{Deserialize, Serialize};

pub use access_token::*;
pub use api::*;
pub use clients::*;
pub use error::*;
pub(crate) use i18n::*;
pub use link::*;
pub use namespace::*;
#[cfg(feature = "reqwest")]
pub use reqwest_client::*;

include!(concat!(env!("OUT_DIR"), "/mod.rs"));

mod access_token;
mod api;
mod clients;
mod error;
mod link;
mod namespace;
pub mod oauth;
#[cfg(feature = "reqwest")]
mod reqwest_client;

#[allow(dead_code)]
fn serialize_not_nan(value: &NotNan<f64>) -> Result<serde_lite::Intermediate, serde_lite::Error> {
    value.into_inner().serialize()
}

#[allow(dead_code)]
fn deserialize_not_nan(value: &serde_lite::Intermediate) -> Result<NotNan<f64>, serde_lite::Error> {
    f64::deserialize(value).and_then(|v| NotNan::new(v).map_err(serde_lite::Error::custom))
}
