use std::ops::DerefMut;
use std::sync::Arc;

use leptos::context::{provide_context, use_context};
use leptos::prelude::{ArcRwSignal, RwSignal, Writeable};
use leptos::prelude::guards::WriteGuard;
use i18n::LocalizedString;
use planner::fights::dragonflight::aberrus::kazzara::Kazzara;

use planner::{PlannerCharacterTemplate, PlannerRealm, PlannerState};

pub fn provide_planner_state_context() {
    let planner_state = PlannerState::new(vec![
        Arc::new(Kazzara::mythic(None, None)),
    ]);
    let planner_state: ArcRwSignal<PlannerState> = ArcRwSignal::new(planner_state);
    provide_context(planner_state);
}

pub fn use_planner() -> RwSignal<PlannerState> {
    use_context::<ArcRwSignal<PlannerState>>().unwrap().into()
}
