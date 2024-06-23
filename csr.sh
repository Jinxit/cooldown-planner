set -eu
export CARGO_TARGET_DIR=../../target_csr
export CARGO_TERM_COLOR=always
export RUST_LOG=info,sqlx=warn
export RUSTFLAGS=--cfg=web_sys_unstable_apis
(cd web/front-csr && trunk serve --color always)
