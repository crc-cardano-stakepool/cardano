# Cardano Rust CLI

[![Top Language](https://img.shields.io/github/languages/top/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano)
[![Release Version](https://img.shields.io/github/v/release/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/releases)
[![Commits/month](https://img.shields.io/github/commit-activity/m/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/graphs/commit-activity)
[![Last Commit](https://img.shields.io/github/last-commit/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/graphs/commit-activity)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/crc-cardano-stakepool/cardano/cardano?label=cardano&logo=github&style=flat)](https://github.com/crc-cardano-stakepool/cardano/actions?query=workflow:cardano)
[![Last release date](https://img.shields.io/github/release-date/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/releases)
[![Github open PRs](https://img.shields.io/github/issues-pr-raw/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/pulls)
[![Github closed PRs](https://img.shields.io/github/issues-pr-closed/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/pulls?q=is%3Apr+is%3Aclosed)
[![Github Issues](https://img.shields.io/github/issues-raw/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/issues)
[![Github closed Issues](https://img.shields.io/github/issues-closed/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/issues?q=is%3Aissue+is%3Aclosed)
[![Contributors](https://img.shields.io/github/contributors/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/graphs/contributors)
[![Followers](https://img.shields.io/github/followers/crc-cardano-stakepool?style=flat)](https://github.com/crc-cardano-stakepool?tab=followers)
[![Forks](https://img.shields.io/github/forks/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/network/members)
[![Stars](https://img.shields.io/github/stars/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/stargazers)
[![Watchers](https://img.shields.io/github/watchers/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/watchers)
[![Twitter Follow](https://img.shields.io/twitter/follow/clemenscodes?logo=twitter&style=flat)](https://twitter.com/clemenscodes)
[![License](https://img.shields.io/github/license/crc-cardano-stakepool/cardano?style=flat)](https://github.com/crc-cardano-stakepool/cardano/blob/master/LICENSE)

## About

Cardano Rust CLI is a tool to interact with cardano components more easily.

As of now, interactions with cardano components is a bit clunky and relies on lots of shell scripts.

To improve this experience, this tool will do the heavy lifting by utilizing the components under the hood and asking for what you want to do.

Whether you are a beginner in the Cardano ecosystem, an SPO or a builder, you can use this tool to have a nicer experience and build more tools for Cardano.

## Available features

- Checking the correct build dependencies for each component
- Building the desired components from scratch
- Updating them to the latest version if out of date

## Future features

- Manage wallets
- Creating transactions
- Minting assets
- SPO utilities
- Setup a plutus development environment
- Explore the blockchain

## Installation

See [installation guide](INSTALL.md)

## Usage

See [usage guide](USAGE.md)

## Documentation

    cargo doc --workspace --no-deps --open
