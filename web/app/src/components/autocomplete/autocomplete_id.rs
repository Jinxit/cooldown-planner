use std::fmt::{Display, Formatter};

use leptos::prelude::serializers::Str;
use leptos::prelude::SharedValue;
use nanoid::nanoid;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AutocompleteId(String);

impl AutocompleteId {
    pub fn new() -> Self {
        let id: SharedValue<String, Str> = SharedValue::new_str(|| nanoid!());
        Self(id.into_inner())
    }
}

impl Display for AutocompleteId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for AutocompleteId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for AutocompleteId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
