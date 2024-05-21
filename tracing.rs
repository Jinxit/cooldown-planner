use leptos::prelude::*;
use tracing_subscriber::{fmt, util::SubscriberInitExt};
use tracing_subscriber_wasm::MakeConsoleWriter;

fn main() {
    console_error_panic_hook::set_once();
    fmt()
        .with_max_level(tracing::Level::TRACE)
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
        .with_ansi(false)
        .pretty()
        .finish()
        .init();
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = signal(cx, 0);
    view! { cx,
        <button on:click=move |_| {
            tracing::debug!("clicked button");
            set_count.update(|n| *n += 1)
        }>{move || {
            tracing::debug!("showing count {}", count.get_untracked());
            count.get()
        }}</button>
    }
}