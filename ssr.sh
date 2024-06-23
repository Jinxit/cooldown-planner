set -eu
#leptosfmt web
#cargo sweep --time 10
#cargo sweep --toolchains nightly
export CARGO_TERM_COLOR=always
export RUST_LOG=info,sqlx=warn
export RUST_BACKTRACE=1
export RUSTFLAGS=--cfg=web_sys_unstable_apis
mkdir -p target
(cd web/storage && \
  DATABASE_URL=sqlite:../../target/storage.sqlite sqlx database setup)
cargo leptos serve $@
