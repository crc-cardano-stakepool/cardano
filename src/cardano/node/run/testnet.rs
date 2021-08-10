use structopt::StructOpt;
use super::run_config::RunConfig;

#[derive(Debug, StructOpt)]
#[structopt(name = "testnet", about = "Run cardano node in testnet")]
pub struct TestnetCommand {
    #[structopt(subcommand)]
    config: Option<RunConfig>
}