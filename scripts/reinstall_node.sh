#!/bin/sh

CARDANO_BIN="$HOME/.local/bin/cardano-node"

rm "$CARDANO_BIN"

cardano node install
