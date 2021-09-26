use crate::cli::{InstallCommand, NodeCommand, UninstallCommand};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cardano", about = "Manage cardano components")]
pub enum CardanoCommand {
    Node(NodeCommand),
    Install(InstallCommand),
    Uninstall(UninstallCommand),
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
    // Update(UpdateCommand),
    // Config(ConfigCommand),
}
