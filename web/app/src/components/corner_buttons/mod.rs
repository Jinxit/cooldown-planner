use leptos::prelude::*;

mod copy_button;
mod lock_button;

use copy_button::*;
use fight_domain::Lookup;
use lock_button::*;
use std::sync::Arc;

#[component]
pub fn CornerButtons(
) -> impl IntoView {
    view! {
        <div
            class="flex"
            style:grid-column-start="attack_name"
            style:grid-column-end="attack_timer"
            style:grid-row-start="character_name"
            style:grid-row-end="character_spells"
        >
            <CopyButton/>
            <LockButton/>
        </div>
    }
}
