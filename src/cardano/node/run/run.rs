
use structopt::StructOpt;
use super::mainnet::MainnetCommand;
use super::testnet::TestnetCommand;

#[derive(Debug, StructOpt)]
#[structopt(name = "run", about = "Run cardano node in mainnet or testnet")]
pub enum RunCommand {
    Mainnet {
        #[structopt(subcommand, about = "The run configuration for mainnet")]
        mainnet: MainnetCommand,
    },
    Testnet {
        #[structopt(subcommand, about = "The run configuration for testnet")]
        testnet: TestnetCommand,
    },
}

