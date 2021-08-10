use super::super::NodeCommand;
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
            RunCommand::Mainnet(config) => RunCommand::mainnet(config),
            RunCommand::Testnet(config) => RunCommand::testnet(config),
        }
    }

    fn mainnet(config: RunConfig) {
        println!("The config to run node in mainnet: {:#?}", config);
        NodeCommand::check_node_version();
    }

    fn testnet(config: RunConfig) {
        println!("The config to run node in testnet: {:#?}", config);
        NodeCommand::check_node_version();
    }
}
