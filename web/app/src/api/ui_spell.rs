use fight_domain::{Identifier, LookupKey, SpellUuid, TimeStep};
use leptos::prelude::*;
use ordered_float::NotNan;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiSpell {
    pub uuid: SpellUuid,
    pub name: String,
    pub icon_text: Option<String>,
    pub power: NotNan<f64>,
    pub cooldown: TimeStep,
    pub cast_time: TimeStep,
    pub identifier: Identifier,
    pub charges: usize,
    pub exclusive_with: BTreeSet<Identifier>,
    pub enabled: bool,
    pub minor: bool,
}

impl LookupKey for UiSpell {
    type Key = SpellUuid;

    fn lookup_key(&self) -> &Self::Key {
        &self.uuid
    }
}
