#![allow(incomplete_features)]
#![allow(async_fn_in_trait)]
#![allow(clippy::module_inception)]

include!(concat!(env!("OUT_DIR"), "/mod.rs"));

mod api;
pub use api::*;
use ordered_float::NotNan;
use serde_lite::{Deserialize, Serialize};

mod clients;
pub use clients::*;
mod error;
pub use error::*;
mod namespace;
pub use namespace::*;
mod locale;
pub use locale::*;
mod region;
pub use region::*;
mod localized_string;
pub use localized_string::*;
mod link;
pub use link::*;
pub mod oauth;
#[cfg(feature = "reqwest")]
mod reqwest_client;
#[cfg(feature = "reqwest")]
pub use reqwest_client::*;

#[allow(dead_code)]
fn serialize_not_nan(value: &NotNan<f64>) -> Result<serde_lite::Intermediate, serde_lite::Error> {
    value.into_inner().serialize()
}

#[allow(dead_code)]
fn deserialize_not_nan(value: &serde_lite::Intermediate) -> Result<NotNan<f64>, serde_lite::Error> {
    f64::deserialize(value).and_then(|v| NotNan::new(v).map_err(serde_lite::Error::custom))
}
