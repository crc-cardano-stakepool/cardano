#!/bin/sh

cargo fmt
cargo test --release --target x86_64-unknown-linux-gnu

