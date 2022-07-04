#!/bin/sh

rm "$(which cardano-node)"
cargo install --path src/bin
