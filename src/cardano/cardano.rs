pub use crate::cardano::*;
use cardano::node::NodeCommand;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cardano", about = "Manage cardano components")]
pub enum CardanoCommand {
    Node(NodeCommand),
    // Cli(CliCommand),
    // Wallet(WalletCommand),
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
    // Install(InstallCommand),
    // Update(UpdateCommand),
    // Config(ConfigCommand),
    // Uninstall(UninstallCommand),
}
