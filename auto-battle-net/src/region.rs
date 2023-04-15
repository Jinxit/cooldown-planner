use serde_lite::{Deserialize, Error, Intermediate, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[allow(dead_code)]
#[derive(
    Display,
    AsRefStr,
    EnumIter,
    EnumString,
    SerializeDisplay,
    DeserializeFromStr,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub enum Region {
    //#[strum(serialize = "apac")]
    //AsiaPacific, // replaces KR and TW // or not apparently??
    #[strum(serialize = "eu")]
    Europe,
    #[strum(serialize = "kr")]
    Korea,
    #[strum(serialize = "tw")]
    Taiwan,
    #[strum(serialize = "us")]
    UnitedStates,
    //#[strum(serialize = "cn")]
    //China,
}

impl Serialize for Region {
    fn serialize(&self) -> Result<Intermediate, Error> {
        self.to_string().serialize()
    }
}

impl Deserialize for Region {
    fn deserialize(val: &Intermediate) -> Result<Self, Error>
    where
        Self: Sized,
    {
        String::deserialize(val).and_then(|s| Region::from_str(&s).map_err(Error::custom))
    }
}
