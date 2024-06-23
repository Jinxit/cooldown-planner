use serde::{Deserialize, Serialize};

use i18n::LocalizedString;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlannerRealm {
    pub name: LocalizedString,
    pub slug: String,
}
