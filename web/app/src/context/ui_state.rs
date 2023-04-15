use leptos::*;

use crate::api::ui_state::UiState;

pub fn provide_ui_state_context() {
    provide_context(UiState::new());
}
