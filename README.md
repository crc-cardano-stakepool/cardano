# Cardano Rust CLI

[![Top Language](https://img.shields.io/github/languages/top/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano)
[![Followers](https://img.shields.io/github/followers/cardano-rust-cli?style=flat)](https://github.com/cardano-rust-cli?tab=followers)
[![Forks](https://img.shields.io/github/forks/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano/network/members)
[![Stars](https://img.shields.io/github/stars/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano/stargazers)
[![Watchers](https://img.shields.io/github/watchers/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano/watchers)
[![Commits/month](https://img.shields.io/github/commit-activity/m/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano/graphs/commit-activity)
[![Last Commit](https://img.shields.io/github/last-commit/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano/graphs/commit-activity)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/cardano-rust-cli/cardano/cardano-rust-cli?label=cardano-rust-cli&logo=github&style=flat)](https://github.com/cardano-rust-cli/cardano/actions?query=workflow:cardano-rust-cli)
[![Github Issues](https://img.shields.io/github/issues-raw/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano/issues)
[![Github open PRs](https://img.shields.io/github/issues-pr-raw/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano/pulls)
[![Contributors](https://img.shields.io/github/contributors/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano/graphs/contributors)
[![License](https://img.shields.io/github/license/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano/blob/master/LICENSE)
[![Repo Size](https://img.shields.io/github/repo-size/cardano-rust-cli/cardano?style=flat)](https://github.com/cardano-rust-cli/cardano)

## Prerequisites

    sudo apt update
    sudo apt upgrade
    sudo apt install curl git libssl-dev build-essential pkg-config
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env

## Install from source to $HOME/.cargo/bin/cardano

    git clone https://github.com/crc-cardano-stakepool/cardano.git
    cd cardano
    cargo install --path .

## Install precompiled binary to /usr/bin/cardano

    git clone https://github.com/crc-cardano-stakepool/cardano.git
    cd cardano
    ./install.sh

## Usage

    cardano <SUBCOMMAND>

## Flags

    -h, --help       Prints help information
    -V, --version    Prints version information

## Subcommands

    help       Prints this message or the help of the given subcommand(s)
    install    Install cardano components
    node       Manage cardano nodes

## Documentation

    cargo doc --open
