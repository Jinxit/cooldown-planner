use crate::region::Region;
use std::fmt::{Display, Formatter};
use strum_macros::{AsRefStr, Display};

#[derive(Copy, Clone, Debug)]
pub struct Namespace {
    pub category: NamespaceCategory,
    pub region: Region,
}

impl Namespace {
    pub fn new(category: NamespaceCategory, region: Region) -> Self {
        Self { category, region }
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.category, self.region)
    }
}

#[allow(dead_code)]
#[derive(Display, AsRefStr, Debug, Copy, Clone)]
pub enum NamespaceCategory {
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "dynamic")]
    Dynamic,
    #[strum(serialize = "profile")]
    Profile,
}
