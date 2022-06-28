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
            // CardanoCommand::Tx(command) => TxCommand::exec(command).await,
            // CardanoCommand::Mint(command) => MintCommand::exec(command).await,
            // CardanoCommand::Address(command) => AddressCommand::exec(command).await,
            // CardanoCommand::Db(command) => DbCommand::exec(command).await,
            // CardanoCommand::Graphql(command) => GraphqlCommand::exec(command).await,
            // CardanoCommand::Ledger(command) => LedgerCommand::exec(command).await,
            // CardanoCommand::Rosetta(command) => RosettaCommand::exec(command).await,
            // CardanoCommand::Plutus(command) => PlutusCommand::exec(command).await,
            // CardanoCommand::Marlowe(command) => MarloweCommand::exec(command).await,
            // CardanoCommand::Explorer(command) => ExplorerCommand::exec(command).await,
            // CardanoCommand::Smash(command) => SmashCommand::exec(command).await,
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
    // Tx(TxCommand),
    // Mint(MintCommand),
    // Address(AddressCommand),
    // Db(DbCommand),
    // Graphql(GraphqlCommand),
    // Ledger(LedgerCommand),
    // Rosetta(RosettaCommand),
    // Plutus(PlutusCommand),
    // Marlowe(MarloweCommand),
    // Explorer(ExplorerCommand),
    // Smash(SmashCommand),
    // Config(ConfigCommand),
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
