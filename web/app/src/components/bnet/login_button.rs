use crate::{serverfns::battle_net_login_url, reactive::resource_ext::ResourceGetExt};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use url::Url;

#[component]
pub fn LoginButton() -> impl IntoView {
    let current_url = move || {
        Url::parse(&format!(
            "http://localhost:3000{}{}",
            use_location().pathname.get(),
            use_location().search.get(),
        ))
        .unwrap()
    };

    let login_url_response = create_local_resource(current_url, move |current_url| async move {
        battle_net_login_url(current_url).await.unwrap_or(None)
    });

    view! {
        <Suspense fallback=move || {
            view! {  <p>"..."</p> }
        }>
            {move || {
                login_url_response
                    .get().flatten()
                                .map(|url| url.to_string())
                                .map(|url| {
                                    view! {  <a href=url>"Login"</a> }
                                })
            }}
        </Suspense>
    }
}
