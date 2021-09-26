use crate::cli::{NodeCommand, WalletCommand};
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Install cardano components")]
pub enum InstallCommand {
    #[structopt(about = "Installs the latest cardano-node")]
    Node,
    #[structopt(about = "Installs the latest cardano-wallet")]
    Wallet,
}

impl InstallCommand {
    pub async fn exec(cmd: InstallCommand) -> Result<()> {
        match cmd {
            InstallCommand::Node => NodeCommand::install_node().await?,
            InstallCommand::Wallet => WalletCommand::install_wallet().await?,
        }
        Ok(())
    }
}
