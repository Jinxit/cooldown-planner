use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Link {
    href: String,
}
