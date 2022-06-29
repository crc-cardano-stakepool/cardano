#!/bin/sh

cargo fmt &&
cargo clippy --workspace &&
cargo doc --workspace --no-deps &&
cargo test --lib --bins --release --target x86_64-unknown-linux-gnu && 
cargo build --release --target x86_64-unknown-linux-gnu
