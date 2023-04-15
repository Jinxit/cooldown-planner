use crate::misc::flatten_ok::FlattenOk;
use crate::reactive::resource_ext::ResourceGetExt;
use crate::serverfns::region_realms;
use auto_battle_net::game_data::realm::realms_index::{Realms, RealmsIndexRequest};
use auto_battle_net::BattleNetClientAsync;
use itertools::Itertools;
use leptos::*;
use tracing::warn;

use super::PlannerContext;

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
    let planner_context = expect_context::<PlannerContext>();
    let realms = create_resource(planner_context.region, move |region| async move {
        region_realms(region).await
    });
    provide_context(RegionRealms(Signal::derive(move || {
        realms.get().flatten_ok().unwrap_or_default()
    })));
}
