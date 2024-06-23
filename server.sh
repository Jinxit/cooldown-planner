set -eu
export CARGO_TERM_COLOR=always
export RUST_LOG=info,sqlx=warn
export RUST_BACKTRACE=1
export RUSTFLAGS=--cfg=web_sys_unstable_apis
export LEPTOS_SITE_ADDR=127.0.0.1:8080
mkdir -p target
(cd web/storage && \
  DATABASE_URL=sqlite:../../target/storage.sqlite sqlx database setup)
cargo watch --ignore .idea --ignore web/css --ignore web/assets --ignore web/front-csr --ignore '*~' --why -- cargo run -p server-package --no-default-features