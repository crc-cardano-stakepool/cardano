use crate::RunConfig;
use anyhow::Result;
use clap::{Args, Subcommand};
use console::Emoji;
use lib::{is_bin_installed, print, print_emoji, Component};

#[derive(Debug, Args)]
pub struct Run {
    #[clap(subcommand)]
    command: RunCommand,
}

#[derive(Debug, Subcommand)]
pub enum RunCommand {
    /// Run cardano-node in mainnet
    Mainnet(RunConfig),
    /// Run cardano-node in testnet
    Testnet(RunConfig),
}

impl RunCommand {
    pub async fn exec(cmd: Run) -> Result<()> {
        match cmd.command {
            RunCommand::Mainnet(config) => RunCommand::mainnet(config).await,
            RunCommand::Testnet(config) => RunCommand::testnet(config).await,
        }
    }

    async fn mainnet(config: RunConfig) -> Result<()> {
        let output: String = format!("The config to run node in mainnet: {config:#?}");
        print("white", &output)?;
        if is_bin_installed(Component::Node).await? {
            print_emoji("green", "Proceeding to run node in mainnet", Emoji("💪", ""))
        } else {
            print("red", "cardano-node is not installed")
        }
    }

    async fn testnet(config: RunConfig) -> Result<()> {
        let output: String = format!("The config to run node in testnet: {config:#?}");
        print("white", &output)?;
        if is_bin_installed(Component::Node).await? {
            print_emoji("green", "Proceeding to run node in testnet", Emoji("🔍", ""))
        } else {
            print("red", "cardano-node is not installed")
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn test_mainnet() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_testnet() {
        unimplemented!();
    }
}
