use leptos::*;

#[component]
pub fn BlankSuspense(children: ChildrenFn) -> impl IntoView {
    let children = store_value(children);
    view! {
        <Suspense fallback=|| ()>
            {move || children.with_value(|value| value())}
        </Suspense>
    }
}
