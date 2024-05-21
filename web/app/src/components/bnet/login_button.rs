use crate::serverfns::battle_net_login_url;
use leptos::prelude::*;
use leptos_router::{hooks::use_location, *};
use tracing::warn;
use url::Url;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn LoginButton() -> impl IntoView {
    let current_url = Signal::derive(move || {
        Url::parse(&format!(
            "http://localhost:3000{}{}",
            "", //use_location().pathname.get(),
            "", //use_location().search.get(),
        ))
        .unwrap()
    });

    let (url, set_url) = signal::<Option<Url>>(None);

    Effect::new(move |_| {
        spawn_local(async move {
            let new_url = battle_net_login_url(current_url()).await.unwrap_or(None);
            set_url.set(new_url)
        });
    });

    let login_url_response = AsyncDerived::new(move || async move {
        //battle_net_login_url(current_url()).await.unwrap_or(None)
        Some(Url::parse("https://example.com").unwrap())
    });

    view! {
        <Suspense fallback=move || {
            view! { <p>"..."</p> }
        }>
            {move || {
                url.get()
                    .map(|url| url.to_string())
                    .map(|url| {
                        view! { <a href=url>"Login"</a> }
                    })
            }}

        </Suspense>
    }
}
