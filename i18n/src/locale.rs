use std::borrow::Cow;

//use serde_lite::{Deserialize, Error, Intermediate, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, VariantNames};

#[allow(dead_code)]
#[derive(
    Display,
    AsRefStr,
    EnumIter,
    VariantNames,
    EnumString,
    SerializeDisplay,
    DeserializeFromStr,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
)]
pub enum Locale {
    #[strum(serialize = "zh_CN", serialize = "zhCN")]
    ChineseSimplified,
    #[strum(serialize = "zh_TW", serialize = "zhTW")]
    ChineseTraditional,
    #[strum(serialize = "en_GB", serialize = "enGB")]
    EnglishGreatBritain,
    #[strum(serialize = "en_US", serialize = "enUS")]
    EnglishUnitedStates,
    #[strum(serialize = "fr_FR", serialize = "frFR")]
    French,
    #[strum(serialize = "de_DE", serialize = "deDE")]
    German,
    #[strum(serialize = "it_IT", serialize = "itIT")]
    Italian,
    #[strum(serialize = "ko_KR", serialize = "koKR")]
    Korean,
    #[strum(serialize = "pt_BR", serialize = "ptBR")]
    Portuguese,
    #[strum(serialize = "ru_RU", serialize = "ruRU")]
    Russian,
    #[strum(serialize = "es_MX", serialize = "esMX")]
    SpanishMexico,
    #[strum(serialize = "es_ES", serialize = "esES")]
    SpanishSpain,
}

impl From<Cow<'static, str>> for Locale {
    fn from(value: Cow<'static, str>) -> Self {
        value.parse().unwrap()
    }
}
