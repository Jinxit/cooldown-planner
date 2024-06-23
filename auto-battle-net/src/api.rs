use std::fmt::Debug;
use std::hash::Hash;

use http::Uri;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Region;

pub trait BattleNetRequest:
    Serialize + DeserializeOwned + Clone + Debug + PartialEq + Hash
{
    type Response: Serialize + DeserializeOwned + Clone + Debug + PartialEq + Hash;

    fn uri(&self, region: Region) -> Uri;
    fn is_user_dependent() -> bool;
}
