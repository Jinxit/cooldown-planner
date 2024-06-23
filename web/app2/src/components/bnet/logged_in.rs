use leptos::*;

use crate::serverfns::is_logged_in;

#[component]
pub fn LoggedIn(#[prop(into)] fallback: ViewFn, children: ChildrenFn) -> impl IntoView {
    let is_logged_in = create_local_resource_with_initial_value(
        move || (),
        move |_| async move {
            is_logged_in().await.unwrap_or(false)
        },
        Some(false)
    );
    let children = StoredValue::new(children);
    // no stored value for fallback

    view! {
        <Suspense>
            {
                let fallback = fallback.clone();
                move || {
                if is_logged_in.get().unwrap_or(false) {
                    children.with_value(|c| c()).into_view()
                } else {
                    fallback.run().into_view()
                }
            }}
        </Suspense>
    }
}
