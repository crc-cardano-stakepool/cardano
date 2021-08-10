
use structopt::StructOpt;
use super::run_config::RunConfig;

#[derive(Debug, StructOpt)]
#[structopt(name = "mainnet", about = "Run cardano node in mainnet")]
pub struct MainnetCommand {
    #[structopt(subcommand)]
    config: Option<RunConfig>
}

