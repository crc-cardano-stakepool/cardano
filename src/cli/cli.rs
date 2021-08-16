use crate::cli::cardano::install::InstallCommand;
use crate::cli::cardano::node::NodeCommand;
use crate::cli::cardano::uninstall::UninstallCommand;
use crate::cli::cardano::CardanoCommand;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cardano", about = "Manage cardano components")]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: CardanoCommand,
}

impl Cli {
    pub async fn start(command: CardanoCommand) -> Result<()> {
        match command {
            CardanoCommand::Node(command) => NodeCommand::exec(command).await?,
            CardanoCommand::Install(command) => InstallCommand::exec(command).await?,
            CardanoCommand::Uninstall(command) => UninstallCommand::exec(command).await?,
            // CardanoCommand::Cli(command) => CliCommand::exec(command).await,
            // CardanoCommand::Wallet(command) => WalletCommand::exec(command).await,
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
            // CardanoCommand::Update(command) => UpdateCommand::exec(command).await,
        }
        Ok(())
    }
}
