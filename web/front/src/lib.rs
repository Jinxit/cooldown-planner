use wasm_bindgen::prelude::wasm_bindgen;

use app_package::*;
use tracing::info;
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
        info!("hydrate mode - hydrating");

        // there are now distinct functions for hydrating and CSR mounting, as opposed to features
        // changing the behavior
        leptos::mount::hydrate_body(App);
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
