use auto_battle_net::oauth::user_authentication::user_info::{UserInfoRequest, UserInfoResponse};
use auto_battle_net::{BattleNetClientAsync, Locale, Namespace, NamespaceCategory, Region};
use leptos::prelude::*;
use leptos_router::*;

use crate::reactive::async_ext::ReadyOrReloading;
use crate::serverfns::is_logged_in;

#[component]
pub fn LoggedIn(#[prop(into)] fallback: ViewFn, children: ChildrenFn) -> impl IntoView {
    let is_logged_in = Resource::new(
        move || (),
        move |_| async move { is_logged_in().await.unwrap_or(false) },
    );

    view! {
        <Suspense fallback=|| "LoggedIn Suspense">
            <Show
                when=move || is_logged_in.ready_or_reloading().unwrap_or(false)
                fallback=fallback.clone()
            >
                {children()}
            </Show>
        </Suspense>
    }
}
