extern crate lib;
use crate::{NodeArgs, NodeCommand, WalletArgs, WalletCommand};
use anyhow::Result;
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use clap_complete::{generate, Generator};
use ctrlc::set_handler;
use human_panic::setup_panic;
use lib::update_cli;
pub mod node;
pub use node::*;
pub mod wallet;
pub use wallet::*;

#[derive(Debug, Parser)]
#[clap(about = "Manage cardano components", version)]
pub struct Cli {
    /// If provided, outputs the completion file for given shell
    #[clap(long = "generate", arg_enum, value_parser)]
    pub generator: Option<Shell>,
    #[clap(subcommand)]
    pub command: Option<CardanoCommand>,
}

impl Cli {
    pub async fn start(command: CardanoCommand) -> Result<()> {
        match command {
            CardanoCommand::Node(command) => NodeCommand::exec(command).await,
            CardanoCommand::Wallet(command) => WalletCommand::exec(command).await,
            CardanoCommand::Update => update_cli().await,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum CardanoCommand {
    /// Manage cardano nodes
    Node(NodeArgs),
    /// Manage cardano wallets
    Wallet(WalletArgs),
    /// Updates the CLI
    Update,
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_panic!();
    set_handler(|| println!("Initialize Ctrl-C handler")).expect("Error setting Ctrl-C handler");
    let cli = Cli::parse();
    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        eprintln!("Generating completion file for {:?}...", generator);
        print_completions(generator, &mut cmd);
        Ok(())
    } else if let Some(command) = cli.command {
        Cli::start(command).await
    } else {
        Ok(())
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
