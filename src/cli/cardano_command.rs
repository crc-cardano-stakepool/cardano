use crate::{NodeArgs, WalletArgs};
use clap::Subcommand;

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
