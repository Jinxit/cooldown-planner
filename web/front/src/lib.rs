use wasm_bindgen::prelude::wasm_bindgen;

use app_package::*;
use leptos::*;
use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    fmt()
        .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
        // For some reason, if we don't do this in the browser, we get
        // a runtime error.
        .without_time()
        .with_ansi(false)
        .init();

    if decode_request(web_sys::window().unwrap()) != Some("no-hydrate".to_string()) {
        log!("hydrate mode - hydrating");

        mount_to_body(|| {
            view! {  <App/> }
        });
    }
}

fn decode_request(window: web_sys::Window) -> Option<String> {
    Some(
        window
            .location()
            .search()
            .ok()?
            .trim_start_matches('?')
            .to_owned(),
    )
}
