use std::fmt::Debug;
use std::hash::Hash;

use leptos::prelude::*;

use fight_domain::LookupKey;
pub use local_resource::*;
use memo::Memoize;

mod local_resource;
pub mod memo;

