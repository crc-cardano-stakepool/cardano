#!/bin/sh

cargo install --path src/bin

CARDANO_BIN="$HOME/.local/bin/cardano-node"

rm "$CARDANO_BIN"

cardano node install -v
