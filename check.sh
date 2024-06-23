set -eu
export CARGO_TERM_COLOR=always
export RUST_LOG=info,sqlx=warn
export RUST_BACKTRACE=1
export RUSTFLAGS=--cfg=web_sys_unstable_apis
export LEPTOS_SITE_ADDR=127.0.0.1:8080
cargo check --workspace
