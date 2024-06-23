use leptos::either::Either;
use leptos::prelude::*;
use i18n::LocalizedString;
use crate::context::UserContext;
use crate::misc::localized_string_with_context::LocalizedStringWithContext;

#[component]
pub fn NameNormal(name: Option<LocalizedString>, realm: Option<LocalizedString>) -> impl IntoView {
    let user = use_context::<UserContext>().unwrap();
    let is_same_realm_as_user = {
        let realm = realm.clone();
        move || {
            let user_realm = user.main_character.get().flatten().map(|mc| mc.realm.name);
            user_realm.is_some() && realm == user_realm
        }
    };
    view! {
        <div class="justify-self-start px-1 group pb-1">
            <span class="group-hover:text-white">{name.localize(user)}</span>
            {move || (!is_same_realm_as_user())
                .then(|| realm.clone())
                .flatten()
                .map(|realm| view! {
                    <span class="text-slate-400 text-sm group-hover:text-slate-300">-{realm.localize(user)}</span>
                })
            }
        </div>
    }
}