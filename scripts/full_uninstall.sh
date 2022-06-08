#!/bin/sh

BIN="/usr/local/bin/cardano"
NODE="$HOME/.local/bin/cardano-node"
CLI="$HOME/.local/bin/cardano-cli"
GHCUP="$HOME/.ghcup"
CABAL="$HOME/.cabal"
CARDANO="$HOME/.cardano"
LIBSODIUM_A="/usr/local/lib/libsodium.a"
LIBSODIUM_SA="/usr/local/lib/libsodium.la"
LIBSODIUM_23_3_0="/usr/local/lib/libsodium.so.23.3.0"
LIBSODIUM_23="/usr/local/lib/libsodium.so.23"
LIBSODIUM_SO="/usr/local/lib/libsodium.so"
LIBSODIUM_PC="/usr/local/lib/pkgconfig/libsodium.pc"

cargo uninstall cardano

sudo rm "$BIN"
rm "$NODE"
rm "$CLI"
rm -rf "$GHCUP"
rm -rf "$CABAL"
rm -rf "$CARDANO"
sudo rm $LIBSODIUM_A
sudo rm $LIBSODIUM_SA
sudo rm $LIBSODIUM_23_3_0
sudo rm $LIBSODIUM_23
sudo rm $LIBSODIUM_SO
sudo rm $LIBSODIUM_PC
