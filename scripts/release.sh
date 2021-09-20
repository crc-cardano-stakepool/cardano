#!/bin/sh

cargo fmt &&
cargo test --release --target x86_64-unknown-linux-gnu &&
cargo build --release --target x86_64-unknown-linux-gnu &&

tar -cvzf ./target/x86_64-unknown-linux-gnu/cardano.tar.gz ./target/x86_64-unknown-linux-gnu/release/cardano
