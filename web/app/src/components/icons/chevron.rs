use leptos::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ChevronDirection {
    Up,
    Down,
}

#[component]
pub fn Chevron(direction: ChevronDirection) -> impl IntoView {
    view! {
        <button class="w-5">
            <div
                class="fa-solid text-xs"
                class:fa-chevron-up=direction == ChevronDirection::Up
                class:fa-chevron-down=direction == ChevronDirection::Down
            ></div>
        </button>
    }
}