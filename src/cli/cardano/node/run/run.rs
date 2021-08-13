use super::super::NodeCommand;
use super::RunConfig;
use crate::cli::utils::Terminal;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Run cardano node")]
pub enum RunCommand {
    #[structopt(about = "Run cardano node in mainnet")]
    Mainnet(RunConfig),
    #[structopt(about = "Run cardano node in testnet")]
    Testnet(RunConfig),
}

impl RunCommand {
    pub async fn exec(cmd: RunCommand) {
        match cmd {
            RunCommand::Mainnet(config) => RunCommand::mainnet(config).await,
            RunCommand::Testnet(config) => RunCommand::testnet(config).await,
        }
    }

    async fn mainnet(config: RunConfig) {
        let output = format!("The config to run node in mainnet: {:#?}", config);
        Terminal::print("white", &output, Emoji("", "")).await;
        if let Ok(true) = NodeCommand::check_node_version().await {
            Terminal::print("green", "Proceeding to run node in mainnet", Emoji("", "")).await;
        }
    }

    async fn testnet(config: RunConfig) {
        let output = format!("The config to run node in testnet: {:#?}", config);
        Terminal::print("white", &output, Emoji("", "")).await;
        if let Ok(true) = NodeCommand::check_node_version().await {
            Terminal::print("green", "Proceeding to run node in testnet", Emoji("", "")).await;
        }
    }
}
