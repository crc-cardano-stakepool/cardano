#!/bin/sh

BIN="cardano"
RELEASE_URL="https://api.github.com/repos/crc-cardano-stakepool/cardano/releases/latest"
LATEST_TAG="$(curl -s "$RELEASE_URL" | jq -r .tag_name)"
LATEST_VERSION="$(echo "$LATEST_TAG" | tr -d 'v' | awk -F '-' '{print $1}')"
DOWNLOAD_URL="https://github.com/crc-cardano-stakepool/cardano/releases/download/$LATEST_TAG/cardano-$LATEST_VERSION-x86_64-unknown-linux-gnu.tar.gz"
DOWNLOAD_PATH="$HOME/Downloads"
DOWNLOAD="$DOWNLOAD_PATH/cardano-$LATEST_VERSION-x86_64-unknown-linux-gnu.tar.gz"
INSTALL_PATH="/usr/local/bin"
wget -q "$DOWNLOAD_URL" -P "$DOWNLOAD_PATH" || { printf "Failed to download latest release\n" && exit 1; }
tar xf "$DOWNLOAD" --directory "$DOWNLOAD_PATH" || { printf "Failed to unzip latest release\n" && rm "$DOWNLOAD" && exit 1; }
printf "Installing $BIN in %s ...\n" "$INSTALL_PATH"
mv "$DOWNLOAD_PATH/cardano" "$INSTALL_PATH" || { printf "Failed to unzip latest release\n" && rm "$DOWNLOAD" && exit 1; }
rm "$DOWNLOAD"
printf "Done\n"
