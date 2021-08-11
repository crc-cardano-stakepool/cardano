# Cardano Rust CLI

## Install from source

    git clone https://github.com/clemenshorn/cardano-rust-cli.git
    cd cardano-rust-cli
    cargo build --release
    sudo mv target/release/cardano /usr/local/bin

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
