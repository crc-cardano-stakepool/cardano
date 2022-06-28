use crate::RunConfig;
use anyhow::Result;
use clap::{Args, Subcommand};
use lib::is_bin_installed;

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
        println!("The config to run node in mainnet: {config:#?}");
        if is_bin_installed("cardano-node").await? {
            println!("Proceeding to run node in mainnet");
        } else {
            println!("cardano-node is not installed");
        }
        Ok(())
    }

    async fn testnet(config: RunConfig) -> Result<()> {
        println!("The config to run node in testnet: {config:#?}");
        if is_bin_installed("cardano-node").await? {
            println!("Proceeding to run node in testnet")
        } else {
            println!("cardano-node is not installed");
        }
        Ok(())
    }
}
