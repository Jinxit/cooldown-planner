use i18n::LocalizedString;
use leptos::prelude::*;
use crate::context::UserContext;
use crate::misc::localized_string_with_context::LocalizedStringWithContext;

#[component]
pub fn SpecName(class: LocalizedString, spec: Option<LocalizedString>) -> impl IntoView {
    let user = use_context::<UserContext>().unwrap();
    view! {
        <div class="justify-self-start px-1 cursor-pointer text-sm">
            {spec.map(|s| view! {
                <span class="mr-1">{s.localize(user)}</span>
            })}
            <span>{class.localize(user)}</span>
        </div>
    }
}
