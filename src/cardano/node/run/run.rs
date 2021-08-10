use super::RunConfig;
use structopt::StructOpt;
use crate::utils::command;

#[derive(Debug, StructOpt)]
#[structopt(about = "Run cardano node in mainnet or testnet")]
pub enum RunCommand {
    Mainnet(RunConfig),
    Testnet(RunConfig),
}

impl RunCommand {
    pub fn exec(cmd: RunCommand) {
        match cmd {
            RunCommand::Mainnet(config) => RunCommand::mainnet(config),
            RunCommand::Testnet(config) => RunCommand::testnet(config),
        }
    }
    fn mainnet(config: RunConfig) {
        println!("The config to run node in mainnet: {:#?}", config);
        println!("Checking for existing cardano-node binary");
        RunCommand::check_node_version();
    }

    fn testnet(config: RunConfig) {
        println!("The config to run node in testnet: {:#?}", config)
    }
    fn check_node_version() {
        command("cardano-node --version");
    }
}
