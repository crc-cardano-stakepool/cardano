# Installation

## Prerequisites

    sudo apt update
    sudo apt upgrade
    sudo apt install git curl wget jq

## Install latest precompiled binary globally

    git clone https://github.com/crc-cardano-stakepool/cardano.git
    cd cardano
    ./install.sh

## Install from source

### Install Rust Toolchain

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env

### Compile

    git clone https://github.com/crc-cardano-stakepool/cardano.git
    cd cardano
    cargo install --path .
