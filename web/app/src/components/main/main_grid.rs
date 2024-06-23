use itertools::Itertools;
use leptos::prelude::*;
use crate::context::use_planner;

#[component]
pub fn MainGrid(tab_open: RwSignal<bool>, children: Children) -> impl IntoView {
    let planner = use_planner();
    let grid_template_columns = move || {
        let planner = planner.read();
        let characters = planner.characters();
        [
            ("attack_name".to_string(), "max-content"),
            ("attack_timer".to_string(), "max-content"),
        ]
            .into_iter()
            .chain(
                characters
                    .iter()
                    .enumerate()
                    .take_while(|(i, _)| *i < characters.len() - 1)
                    .map(|(_, character)| (format!("character_{}", &character.uuid), "auto")),
            )
            .chain(
                [Some(("add_character".to_string(), "auto")), {
                    if !characters.is_empty() {
                        Some((
                            format!("character_{}", &characters.iter().last().unwrap().uuid),
                            "minmax(auto, 12rem)",
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
                planner
                    .read()
                    .attacks()
                    .into_iter()
                    .map(|attack| (format!("attack_{}", &attack.uuid), "auto")),
            )
            .chain([("last_row".to_string(), "1fr")])
            .map(|(name, size)| format!("[{name}] {size}"))
            .join(" ")
    };

    view! {
        <div
            class="grid w-full content-start gap-x-4 overflow-x-hidden overflow-y-visible px-4 text-center"
            style:grid-template-columns=grid_template_columns
            style:grid-template-rows=grid_template_rows
            class=("pt-4", tab_open)
            class=("pt-8", move || !tab_open.get())
        >
            {children()}
        </div>
    }
}