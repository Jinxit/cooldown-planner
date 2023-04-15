use leptos::spawn_local;
use wasm_bindgen_futures::JsFuture;

pub fn write_to_clipboard(s: String, set_failed: impl Fn() + 'static) {
    let clipboard = web_sys::window().unwrap().navigator().clipboard();
    match clipboard {
        Some(clipboard) => spawn_local(async move {
            let promise = clipboard.write_text(&s);
            let result = JsFuture::from(promise).await;
            if let Err(e) = result {
                web_sys::console::log_1(&e);
                set_failed();
            }
        }),
        None => {
            web_sys::console::log_1(&"Clipboard API not supported".into());
            set_failed();
        }
    }
}
