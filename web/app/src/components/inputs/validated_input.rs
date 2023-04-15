use leptos::*;
use std::marker::PhantomData;
use uuid::Uuid;

#[component]
pub fn ValidatedInput<T, F, G>(
    set_result: WriteSignal<T>,
    try_parse: F,
    reformat: G,
    #[prop(into)] label: String,
    #[prop(into)] placeholder: String,
    #[prop(into)] initial_value: T,
) -> impl IntoView
where
    T: 'static,
    F: Fn(&str) -> Option<T> + Clone + 'static,
    G: Fn(&T) -> String + 'static,
{
    let id = Uuid::new_v4().to_string();
    let (value, set_value) = create_signal(reformat(&initial_value));
    let (invalid, set_invalid) = create_signal(false);
    let (is_focused, set_is_focused) = create_signal(false);

    create_effect(move |_| {
        if let Some(result) = try_parse(&value()) {
            set_result(result);
            set_invalid(false);
        } else {
            set_invalid(true);
        }
        if !is_focused() {
            set_value.update(|value| {
                if let Some(result) = try_parse(value) {
                    *value = reformat(&result);
                }
            });
        }
    });

    view! {
        <fieldset>
            <label for=id.clone() class="block text-sm">
                {label}
            </label>
            <div class="relative">
                <input
                    type="text"
                    id=id
                    class="w-20 rounded-md border border-slate-900 bg-slate-700 px-2 py-0.5 pl-6 \
                    text-slate-300 focus-visible:bg-slate-900 focus-visible:outline-none focus-visible:ring-2 "
                    class=("ring-2", invalid)
                    class=("ring-red-400", invalid)
                    class=("focus-visible:ring-red-400", invalid)
                    class=("focus-visible:ring-slate-500", move || !invalid())
                    placeholder=placeholder
                    prop:value=value
                    on:input=move |ev| {
                        let text = event_target_value(&ev);
                        set_value(text);
                    }
                    on:focus=move |_| {
                        set_is_focused(true);
                    }
                    on:blur=move |_| {
                        set_is_focused(false);
                    }
                />
                <div class="absolute left-0 top-0 flex h-full items-center pl-2">
                    <div
                        class="fas fa-hourglass pointer-events-none text-sm"
                        class=("text-slate-400", move || !invalid())
                        class=("text-red-400", invalid)
                    ></div>
                </div>
            </div>
        </fieldset>
    }
}
