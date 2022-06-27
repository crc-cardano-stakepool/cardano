use crate::{CardanoCommand, NodeCommand, UpdateCommand, WalletCommand};
use anyhow::Result;
use clap::Parser;
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[clap(about = "Manage cardano components", version, long_about = None)]
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
            CardanoCommand::Update => UpdateCommand::update().await,
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
