use itertools::Itertools;
use leptos::prelude::*;

use auto_battle_net::game_data::realm::realms_index::Realms;

use crate::misc::flatten_ok::FlattenOk;
use crate::serverfns::region_realms;

use super::UserContext;

#[derive(Copy, Clone, Debug)]
pub struct RegionRealms(Signal<Vec<Realms>>);

impl IntoIterator for RegionRealms {
    type Item = Realms;
    type IntoIter = <Vec<Realms> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.get().into_iter()
    }
}

pub fn provide_region_realms_context() {
    let region = use_context::<UserContext>().unwrap().region.clone();
    let realms = Resource::new_serde(
        region,
        move |region| async move { region_realms(region).await },
    );
    provide_context(RegionRealms(Signal::derive(move || {
        realms.ready_or_reloading().flatten_ok().unwrap_or_default()
    })));
}
