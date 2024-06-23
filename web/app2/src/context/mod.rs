use leptos::*;

mod class_cache;
mod class_spec_index;
mod guild_roster;
mod planner;
mod region_realms;
mod ui_state;
mod workers;

pub use class_cache::{ClassCache, PlayerClass};
pub use class_spec_index::ClassSpecIndex;
pub use guild_roster::GuildRoster;
pub use planner::{PlannerContext, PlannerRealm, PlannerUser};
pub use region_realms::RegionRealms;
pub use workers::with_workers;

pub fn provide() {
    //leptos_query::provide_query_client();
    planner::provide_planner_context();
    region_realms::provide_region_realms_context();
    class_cache::provide_class_cache_context();
    class_spec_index::provide_class_spec_index_context();
    guild_roster::provide_guild_roster_context();
    workers::provide_workers_context();
    ui_state::provide_ui_state_context();
}
