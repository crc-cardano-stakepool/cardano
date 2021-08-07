use structopt::StructOpt;
pub use crate::cardano::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "cardano", about = "A Cardano CLI built with Rust")]
pub struct Cardano {
    #[structopt(subcommand)]
    pub cmd: CardanoCommand,
}

#[derive(Debug, StructOpt)]
pub enum CardanoCommand {
    Node(NodeCommand),
    Cli(CliCommand),
    Wallet(WalletCommand),
    Tx(TxCommand),
    Mint(MintCommand),
    Address(AddressCommand),
    Db(DbCommand),
    Graphql(GraphqlCommand),
    Ledger(LedgerCommand),
    Rosetta(RosettaCommand),
    Plutus(PlutusCommand),
    Marlowe(MarloweCommand),
    Explorer(ExplorerCommand),
    Smash(SmashCommand),
    Install(InstallCommand),
    Update(UpdateCommand),
    Config(ConfigCommand),
    Uninstall(UninstallCommand),
}

impl Cardano {
    pub fn start(cmd: CardanoCommand) {
        match cmd {
            CardanoCommand::Node(cmd) => println!("node command: {:#?}", cmd),
            CardanoCommand::Cli(cmd) => println!("cli command: {:#?}", cmd),
            CardanoCommand::Wallet(cmd) => println!("wallet command: {:#?}", cmd),
            CardanoCommand::Tx(cmd) => println!("tx command: {:#?}", cmd),
            CardanoCommand::Mint(cmd) => println!("mint command: {:#?}", cmd),
            CardanoCommand::Address(cmd) => println!("address command: {:#?}", cmd),
            CardanoCommand::Db(cmd) => println!("db command: {:#?}", cmd),
            CardanoCommand::Graphql(cmd) => println!("graphql command: {:#?}", cmd),
            CardanoCommand::Ledger(cmd) => println!("ledger command: {:#?}", cmd),
            CardanoCommand::Rosetta(cmd) => println!("rosetta command: {:#?}", cmd),
            CardanoCommand::Plutus(cmd) => println!("plutus command: {:#?}", cmd),
            CardanoCommand::Marlowe(cmd) => println!("marlowe command: {:#?}", cmd),
            CardanoCommand::Explorer(cmd) => println!("explorer command: {:#?}", cmd),
            CardanoCommand::Smash(cmd) => println!("smash command: {:#?}", cmd),
            CardanoCommand::Install(cmd) => println!("install command: {:#?}", cmd),
            CardanoCommand::Update(cmd) => println!("update command: {:#?}", cmd),
            CardanoCommand::Config(cmd) => println!("config command: {:#?}", cmd),
            CardanoCommand::Uninstall(cmd) => println!("uninstall command: {:#?}", cmd),
        }
    }
}

