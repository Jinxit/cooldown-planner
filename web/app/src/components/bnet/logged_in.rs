use auto_battle_net::oauth::user_authentication::user_info::{UserInfoRequest, UserInfoResponse};
use auto_battle_net::{BattleNetClientAsync, Locale, Namespace, NamespaceCategory, Region};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::reactive::resource_ext::ResourceGetExt;
use crate::serverfns::is_logged_in;

#[component]
pub fn LoggedIn<F, IV>(fallback: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn() -> IV + Clone + 'static,
    IV: IntoView,
{
    let is_logged_in: Resource<(), bool> = create_local_resource_with_initial_value(
        move || (),
        move |_| async move { is_logged_in().await.unwrap_or(false) },
        Some(false),
    );

    let children = store_value(children);
    view! {
        <Suspense fallback=|| "LoggedIn Suspense" clone:fallback>
            <Show when=move || is_logged_in.get().unwrap_or(false) fallback=fallback.clone()>
                {children.with_value(|children| children())}
            </Show>
        </Suspense>
    }
}
