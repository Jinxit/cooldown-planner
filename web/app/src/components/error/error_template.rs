use leptos::prelude::*;

#[component]
fn ErrorTemplate(#[prop(into)] errors: ArcRwSignal<Errors>) -> impl IntoView {
    let errors = move || errors.get().into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    view! {
        <h1>"Errors"</h1>
        {move || {
            errors()
                .into_iter()
                .map(|error| {
                    view! { <p>"Error: " {error.to_string()}</p> }
                })
                .collect::<Vec<_>>()
        }}
    }
}