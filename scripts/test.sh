#!/bin/sh

cargo fmt &&
cargo clippy &&
cargo test --release --target x86_64-unknown-linux-gnu
