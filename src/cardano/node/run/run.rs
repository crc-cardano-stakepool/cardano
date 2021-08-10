use super::RunConfig;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Run cardano node in mainnet or testnet")]
pub enum RunCommand {
    Mainnet(RunConfig),
    Testnet(RunConfig),
}

impl RunCommand {
    pub fn exec(cmd: RunCommand) {
        match cmd {
            RunCommand::Mainnet(config) => RunConfig::mainnet(config),
            RunCommand::Testnet(config) => RunConfig::testnet(config),
        }
    }
}
