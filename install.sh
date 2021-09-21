#!/bin/sh

RELEASE_URL="https://api.github.com/repos/crc-cardano-stakepool/cardano/releases/latest"
LATEST_TAG="$(curl -s "$RELEASE_URL" | grep tag_name | awk -F ':' '{print $2}' | tr -d '"' | tr -d ',' | tr -d '[:space:]')"
LATEST_VERSION="$(echo "$LATEST_TAG" | tr -d 'v' )"
DOWNLOAD_URL="https://github.com/crc-cardano-stakepool/cardano/releases/download/$LATEST_TAG/cardano-$LATEST_VERSION-x86_64-unknown-linux-gnu.tar.gz"
DIGEST="cardano-$LATEST_VERSION-x86_64-unknown-linux-gnu.tar.gz"
INSTALL_PATH="/usr/bin/cardano"
wget "$DOWNLOAD_URL"
tar xvf "$DIGEST"
sudo cp cardano "$INSTALL_PATH"
