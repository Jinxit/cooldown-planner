use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use url::Url;
use crate::components::login::login_button::LoginButton;

use crate::serverfns::battle_net_login_url;

#[component]
pub fn LoggedIn<C>(children: TypedChildrenFn<C>) -> impl IntoView
    where
        C: IntoView + 'static,
{
    let current_url = move || {
        Url::parse(&format!(
            "http://localhost:3000{}{}",
            "",//use_location().pathname.get(),
            "",//use_location().search.get(),
        ))
            .unwrap()
    };

    let login_url = Resource::new(current_url, move |current_url| async move {
        let login_url = battle_net_login_url(current_url).await?;
        let login_url = login_url.as_ref().map(Url::to_string);
        Ok::<_, ServerFnError>(login_url)
    });

    let children = children.into_inner();

    view! {
        <Suspense>
            {move || Suspend({
                let children = children();
                async move {
                    login_url
                        .await
                        .map(|login_url| {
                            match login_url {
                                Some(login_url) => Either::Left(view! { <LoginButton login_url/> }),
                                None => Either::Right(children),
                            }
                        })
                }
            })}
        </Suspense>
    }
}