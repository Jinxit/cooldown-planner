use convert_case::{Case, Casing};
use leptos::prelude::*;
use i18n::{Locale, LocalizedString};

#[component]
pub fn ClassColorBar(class: Option<LocalizedString>) -> impl IntoView {
    let class_color = format!(
        "cc-{}",
        class
            .map(|c| c.get(Locale::EnglishUnitedStates).to_case(Case::Kebab))
            .unwrap_or("general".to_string())
    );
    const NON_BREAKING_SPACE: &str = "\u{a0}";

    view! {
        <div class=format!("bg-{}", class_color) + " w-0.5 inline-block h-full">
            {NON_BREAKING_SPACE}
        </div>
    }
}