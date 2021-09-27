#!/bin/sh

CARDANO_DIR="$HOME/.cardano"
CARDANO_BIN="$HOME/.cargo/bin/cardano"

rm -rf "$CARDANO_DIR"
rm -rf "$CARDANO_BIN"

cargo install --path .

cardano node install
