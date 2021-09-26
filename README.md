# Cardano Rust CLI

[![License](https://img.shields.io/github/license/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/blob/master/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/crc-cardano-stakepool/cardano/cardano?label=cardano&logo=github&style=flat)](https://github.com/crc-cardano-stakepool/cardano/actions?query=workflow:cardano)
[![Last release date](https://img.shields.io/github/release-date/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/releases)
[![Last Commit](https://img.shields.io/github/last-commit/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/graphs/commit-activity)
[![Top Language](https://img.shields.io/github/languages/top/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano)
[![Followers](https://img.shields.io/github/followers/crc-cardano-stakepool?style=flat)](https://github.com/crc-cardano-stakepool?tab=followers)
[![Forks](https://img.shields.io/github/forks/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/network/members)
[![Stars](https://img.shields.io/github/stars/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/stargazers)
[![Watchers](https://img.shields.io/github/watchers/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/watchers)
[![Github Issues](https://img.shields.io/github/issues-raw/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/issues)
[![Github closed Issues](https://img.shields.io/github/issues-closed/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/issues?q=is%3Aissue+is%3Aclosed)
[![Github open PRs](https://img.shields.io/github/issues-pr-raw/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/pulls)
[![Github closed PRs](https://img.shields.io/github/issues-pr-closed/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/pulls?q=is%3Apr+is%3Aclosed)
[![Commits/month](https://img.shields.io/github/commit-activity/m/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/graphs/commit-activity)
[![Contributors](https://img.shields.io/github/contributors/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/graphs/contributors)
[![Release Version](https://img.shields.io/github/v/release/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/releases)
[![Twitter Follow](https://img.shields.io/twitter/follow/clemenscodes?logo=twitter&style=flat)](https://twitter.com/clemenscodes)

## Prerequisites

    sudo apt update
    sudo apt upgrade
    sudo apt install git curl wget

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

## Usage

    cardano <SUBCOMMAND>

## Flags

    -h, --help       Prints help information
    -V, --version    Prints version information

## Subcommands

    help       Prints this message or the help of the given subcommand(s)
    install    Install cardano components
    node       Manage cardano nodes
    uninstall  Uninstall cardano components

## Documentation

    cargo doc --open
