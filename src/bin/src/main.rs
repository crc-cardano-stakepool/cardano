extern crate lib;
use crate::{NodeArgs, NodeCommand, WalletArgs, WalletCommand};
use anyhow::Result;
use clap::{ColorChoice, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use ctrlc::set_handler;
use human_panic::setup_panic;
use lib::update_cli;
pub mod node;
pub use node::*;
pub mod wallet;
pub use wallet::*;

#[derive(Debug, Parser)]
#[clap(about = "Manage cardano components", version, color = ColorChoice::Never)]
pub struct Cli {
    #[clap(long = "generate", arg_enum, value_parser)]
    pub generator: Option<Shell>,
    #[clap(subcommand)]
    pub command: Option<CardanoCommand>,
}

impl Cli {
    pub async fn exec(command: CardanoCommand) -> Result<()> {
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
    let mut cmd = Cli::command();
    if let Some(generator) = cli.generator {
        let bin_name = cmd.get_name().to_string();
        eprintln!("Generating completion file for {:?}...", generator);
        generate(generator, &mut cmd, bin_name, &mut std::io::stdout());
        Ok(())
    } else if let Some(command) = cli.command {
        Cli::exec(command).await
    } else {
        cmd.print_help()?;
        Ok(())
    }
}
