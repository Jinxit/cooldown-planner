use crate::locale::Locale;
use serde_lite_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq)]
pub struct LocalizedString(Arc<HashMap<Locale, String>>);

impl Ord for LocalizedString {
    fn cmp(&self, other: &Self) -> Ordering {
        Locale::iter()
            .map(|l| self.get(l))
            .cmp(Locale::iter().map(|l| other.get(l)))
    }
}

impl PartialOrd for LocalizedString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl LocalizedString {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        chinese_simplified: &str,
        chinese_traditional: &str,
        english_great_britain: &str,
        english_united_states: &str,
        french: &str,
        german: &str,
        italian: &str,
        korean: &str,
        portuguese: &str,
        russian: &str,
        spanish_mexico: &str,
        spanish_spain: &str,
    ) -> Self {
        Self(Arc::new(
            [
                (Locale::ChineseSimplified, chinese_simplified.to_string()),
                (Locale::ChineseTraditional, chinese_traditional.to_string()),
                (
                    Locale::EnglishGreatBritain,
                    english_great_britain.to_string(),
                ),
                (
                    Locale::EnglishUnitedStates,
                    english_united_states.to_string(),
                ),
                (Locale::French, french.to_string()),
                (Locale::German, german.to_string()),
                (Locale::Italian, italian.to_string()),
                (Locale::Korean, korean.to_string()),
                (Locale::Portuguese, portuguese.to_string()),
                (Locale::Russian, russian.to_string()),
                (Locale::SpanishMexico, spanish_mexico.to_string()),
                (Locale::SpanishSpain, spanish_spain.to_string()),
            ]
            .into_iter()
            .collect(),
        ))
    }

    pub fn get(&self, locale: Locale) -> &str {
        self.0.get(&locale).unwrap()
    }

    pub fn constant(text: &str) -> Self {
        Self::new(
            text, text, text, text, text, text, text, text, text, text, text, text,
        )
    }
}

impl Debug for LocalizedString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "en_US:\"{}\"", self.get(Locale::EnglishUnitedStates))
    }
}

impl Hash for LocalizedString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get(Locale::EnglishUnitedStates).hash(state);
    }
}
