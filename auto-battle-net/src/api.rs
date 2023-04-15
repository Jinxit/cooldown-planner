use crate::region::Region;
use http::Uri;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;

pub trait BattleNetRequest:
    Serialize + DeserializeOwned + Clone + Debug + PartialEq + Hash
{
    type Response: Serialize + DeserializeOwned + Clone + Debug + PartialEq + Hash;

    fn uri(&self, region: Region) -> Uri;
    fn should_cache() -> bool;
}
