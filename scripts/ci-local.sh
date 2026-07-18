#!/usr/bin/env sh
set -eu

printf '%s\n' "==> Rust format"
cargo fmt --all --check

printf '%s\n' "==> Rust source width"
./scripts/check-rust-width.sh

printf '%s\n' "==> whitespace"
./scripts/check-whitespace.py

printf '%s\n' "==> Clippy"
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings

printf '%s\n' "==> rustdoc"
RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --no-deps

printf '%s\n' "==> deterministic Rust tests"
cargo test --locked --workspace --lib --tests

printf '%s\n' "==> documentation tests"
cargo test --locked --workspace --doc

printf '%s\n' "==> attributable browser artifact"
cargo make test-artifact

printf '%s\n' "==> browser adapter tests"
node --test web/tests/*.test.mjs

printf '%s\n' "==> Grafik default gate passed"
