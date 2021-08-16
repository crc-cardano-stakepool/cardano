use crate::cli::install::InstallCommand;
use crate::cli::node::NodeCommand;
use crate::cli::uninstall::UninstallCommand;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cardano", about = "Manage cardano components")]
pub enum CardanoCommand {
    Node(NodeCommand),
    Install(InstallCommand),
    Uninstall(UninstallCommand),
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
    // Update(UpdateCommand),
    // Config(ConfigCommand),
}
