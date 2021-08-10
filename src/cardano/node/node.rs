use structopt::StructOpt;
use super::run::*;

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage cardano nodes")]
pub enum NodeCommand {
    Run {
        #[structopt(subcommand, help = "The run configuration for mainnet")]
        run: RunCommand,
    }
}
