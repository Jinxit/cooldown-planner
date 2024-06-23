use leptos::prelude::*;
use leptos::logging;
use app_package::*;
use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;

#[component]
pub fn Test() -> impl IntoView {
    view! {
        <App/>
    }
}

pub fn main() {
    if web_sys::window().is_some() {
        console_error_panic_hook::set_once();
        fmt()
            .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
            // For some reason, if we don't do this in the browser, we get
            // a runtime error.
            .without_time()
            .with_ansi(false)
            .init();

        logging::log!("csr mode - mounting to body");
        mount_to_body(Test);
    }
}
