use crate::cardano::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cardano", about = "Manage cardano components")]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: CardanoCommand,
}

impl Cli {
    pub async fn start(cmd: CardanoCommand) {
        match cmd {
            CardanoCommand::Node(cmd) => NodeCommand::exec(cmd).await,
            CardanoCommand::Install(cmd) => InstallCommand::exec(cmd).await,
            // CardanoCommand::Cli(cmd) => println!("cli command: {:#?}", cmd),
            // CardanoCommand::Wallet(cmd) => println!("wallet command: {:#?}", cmd),
            // CardanoCommand::Tx(cmd) => println!("tx command: {:#?}", cmd),
            // CardanoCommand::Mint(cmd) => println!("mint command: {:#?}", cmd),
            // CardanoCommand::Address(cmd) => println!("address command: {:#?}", cmd),
            // CardanoCommand::Db(cmd) => println!("db command: {:#?}", cmd),
            // CardanoCommand::Graphql(cmd) => println!("graphql command: {:#?}", cmd),
            // CardanoCommand::Ledger(cmd) => println!("ledger command: {:#?}", cmd),
            // CardanoCommand::Rosetta(cmd) => println!("rosetta command: {:#?}", cmd),
            // CardanoCommand::Plutus(cmd) => println!("plutus command: {:#?}", cmd),
            // CardanoCommand::Marlowe(cmd) => println!("marlowe command: {:#?}", cmd),
            // CardanoCommand::Explorer(cmd) => println!("explorer command: {:#?}", cmd),
            // CardanoCommand::Smash(cmd) => println!("smash command: {:#?}", cmd),
            // CardanoCommand::Update(cmd) => println!("update command: {:#?}", cmd),
            // CardanoCommand::Config(cmd) => println!("config command: {:#?}", cmd),
            // CardanoCommand::Uninstall(cmd) => println!("uninstall command: {:#?}", cmd),
        }
    }
}