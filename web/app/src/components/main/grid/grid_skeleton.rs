use crate::api::{ui_character::UiCharacter, ui_state::UiState};
use fight_domain::{Attack, Character, Lookup};
use itertools::Itertools;
use leptos::{prelude::*, tachys::view::any_view::IntoAny};
use std::sync::Arc;

#[component]
pub fn GridSkeleton(
    #[prop(into)] tab_open: Signal<bool>,
    children: ChildrenFragment,
) -> impl IntoView {
    let ui_state = use_context::<UiState>().unwrap();
    let grid_template_columns = move || {
        let ui_characters = ui_state.ui_characters();
        [
            ("attack_name".to_string(), "max-content"),
            ("attack_timer".to_string(), "max-content"),
        ]
        .into_iter()
        .chain(
            ui_characters
                .iter()
                .enumerate()
                .take_while(|(i, _)| *i < ui_characters.len() - 1)
                .map(|(_, character)| (format!("character_{}", &character.uuid), "auto")),
        )
        .chain(
            [Some(("add_character".to_string(), "2rem")), {
                if !ui_characters.is_empty() {
                    Some((
                        format!(
                            "character_{}",
                            &ui_characters
                                .iter()
                                .nth(ui_characters.len() - 1)
                                .unwrap()
                                .uuid
                        ),
                        "auto",
                    ))
                } else {
                    None
                }
            }]
            .into_iter()
            .flatten(),
        )
        .map(|(name, size)| format!("[{name}] {size}"))
        .join(" ")
    };

    let grid_template_rows = move || {
        [
            ("character_name".to_string(), "auto"),
            ("character_spells".to_string(), "auto"),
        ]
        .into_iter()
        .chain(
            ui_state
                .attacks()
                .into_iter()
                .map(|attack| (format!("attack_{}", &attack.uuid), "auto")),
        )
        .chain([("last_row".to_string(), "1fr")])
        .map(|(name, size)| format!("[{name}] {size}"))
        .join(" ")
    };

    let children = children()
            .nodes
            .into_iter()
            .map(|child| view! { <Suspense>{child}</Suspense> })
            .collect::<Vec<_>>();

    view! {
        <div
            class="font-sans grid w-full content-start gap-x-4 overflow-x-hidden overflow-y-visible px-4 \
            text-center font-medium text-slate-100 transition-[padding]"
            class=("pt-4", tab_open)
            class=("pt-8", move || !tab_open())
            tabindex=-1
            style:grid-template-columns=grid_template_columns
            style:grid-template-rows=grid_template_rows
        >
            {children}
        </div>
    }
}
