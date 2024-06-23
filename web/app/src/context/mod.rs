use leptos::prelude::*;

pub use class_cache::ClassCache;
pub use class_spec_index::{ClassSpecIndex, use_class_spec_index};
pub use guild_roster::{GuildRoster, RosterEntry};
pub use planner_state::use_planner;
pub use user::UserContext;
pub use workers::with_workers;

mod class_cache;
mod class_spec_index;
mod guild_roster;
mod planner_state;
mod user;
mod workers;

pub fn provide() {
    //leptos_query::provide_query_client();
    user::provide_user_context();
    workers::provide_workers_context();
    planner_state::provide_planner_state_context();
    class_cache::provide_class_cache_context();
    class_spec_index::provide_class_spec_index_context();
    guild_roster::provide_guild_roster_context();
}
