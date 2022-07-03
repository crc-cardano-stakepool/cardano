extern crate lib;
use lib::{read_setting, setup_logger, update_cli};

use crate::{NodeArgs, NodeCommand, WalletArgs, WalletCommand};

use anyhow::Result;
use clap::{ColorChoice, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use clap_verbosity_flag::{InfoLevel, Verbosity};

pub mod node;
pub use node::*;
pub mod wallet;
pub use wallet::*;

#[derive(Debug, Parser)]
#[clap(about = "Manage cardano components", version, color = ColorChoice::Never)]
pub struct Cli {
    /// Generate shell a shell completion file
    #[clap(short, long = "generate", arg_enum, value_parser)]
    pub generator: Option<Shell>,
    #[clap(subcommand)]
    pub command: Option<CardanoCommand>,
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
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
    let cli = Cli::parse();
    let log_file = read_setting("log_file")?;
    setup_logger(cli.verbose.log_level_filter(), true, log_file)?;
    human_panic::setup_panic!();
    ctrlc::set_handler(|| log::info!("Initialize Ctrl-C handler")).expect("Error setting Ctrl-C handler");
    let mut cmd = Cli::command();
    if let Some(generator) = cli.generator {
        let bin_name = cmd.get_name().to_string();
        log::info!("Generating completion file for {:?}...", generator);
        generate(generator, &mut cmd, bin_name, &mut std::io::stdout());
        Ok(())
    } else if let Some(command) = cli.command {
        Cli::exec(command).await
    } else {
        cmd.print_help()?;
        Ok(())
    }
}
