#!/bin/sh

cargo fmt &&
cargo clippy --workspace &&
cargo doc --workspace --no-deps --open &&
cargo test --workspace --release --target x86_64-unknown-linux-gnu
