#!/bin/sh

TARGET="x86_64-unknown-linux-gnu"
TAG="0.0.1-alpha"

cargo fmt &&
cargo test --release --target $TARGET &&
cargo build --release --target $TARGET &&
tar -cvzf ./target/$TARGET/cardano-$TAG-$TARGET.tar.gz ./target/$TARGET/release/cardano
