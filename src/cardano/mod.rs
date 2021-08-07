use structopt::StructOpt;
pub mod node;
pub use node::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "cardano", about = "A Cardano CLI built with Rust")]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
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

#[derive(Debug, StructOpt)]
pub enum CliCommand {}

#[derive(Debug, StructOpt)]
pub enum WalletCommand {}

#[derive(Debug, StructOpt)]
pub enum TxCommand {}

#[derive(Debug, StructOpt)]
pub enum MintCommand {}

#[derive(Debug, StructOpt)]
pub enum AddressCommand {}

#[derive(Debug, StructOpt)]
pub enum DbCommand {}

#[derive(Debug, StructOpt)]
pub enum GraphqlCommand {}

#[derive(Debug, StructOpt)]
pub enum LedgerCommand {}

#[derive(Debug, StructOpt)]
pub enum RosettaCommand {}

#[derive(Debug, StructOpt)]
pub enum PlutusCommand {}

#[derive(Debug, StructOpt)]
pub enum MarloweCommand {}

#[derive(Debug, StructOpt)]
pub enum ExplorerCommand {}

#[derive(Debug, StructOpt)]
pub enum SmashCommand {}

#[derive(Debug, StructOpt)]
pub enum InstallCommand {}

#[derive(Debug, StructOpt)]
pub enum UpdateCommand {}

#[derive(Debug, StructOpt)]
pub enum ConfigCommand {}

#[derive(Debug, StructOpt)]
pub enum UninstallCommand {}

impl Cli {
    pub fn start(cmd: Command) {
        match cmd {
            Command::Node(cmd) => println!("node command: {:#?}", cmd),
            Command::Cli(cmd) => println!("cli command: {:#?}", cmd),
            Command::Wallet(cmd) => println!("wallet command: {:#?}", cmd),
            Command::Tx(cmd) => println!("tx command: {:#?}", cmd),
            Command::Mint(cmd) => println!("mint command: {:#?}", cmd),
            Command::Address(cmd) => println!("address command: {:#?}", cmd),
            Command::Db(cmd) => println!("db command: {:#?}", cmd),
            Command::Graphql(cmd) => println!("graphql command: {:#?}", cmd),
            Command::Ledger(cmd) => println!("ledger command: {:#?}", cmd),
            Command::Rosetta(cmd) => println!("rosetta command: {:#?}", cmd),
            Command::Plutus(cmd) => println!("plutus command: {:#?}", cmd),
            Command::Marlowe(cmd) => println!("marlowe command: {:#?}", cmd),
            Command::Explorer(cmd) => println!("explorer command: {:#?}", cmd),
            Command::Smash(cmd) => println!("smash command: {:#?}", cmd),
            Command::Install(cmd) => println!("install command: {:#?}", cmd),
            Command::Update(cmd) => println!("update command: {:#?}", cmd),
            Command::Config(cmd) => println!("config command: {:#?}", cmd),
            Command::Uninstall(cmd) => println!("uninstall command: {:#?}", cmd),
        }
    }
}
