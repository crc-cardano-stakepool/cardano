use super::super::cli::CliCommand;
use super::super::node::NodeCommand;
use super::super::wallet::WalletCommand;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "uninstall", about = "Uninstall cardano components")]
pub enum UninstallCommand {
    #[structopt(about = "Uninstalls cardano-node")]
    Node,
    #[structopt(about = "Uninstalls cardano-cli")]
    Cli,
    #[structopt(about = "Uninstalls cardano-wallet")]
    Wallet,
}

impl UninstallCommand {
    pub async fn exec(cmd: UninstallCommand) -> Result<()> {
        match cmd {
            UninstallCommand::Node => NodeCommand::uninstall_node().await?,
            UninstallCommand::Cli => CliCommand::uninstall_cli()?,
            UninstallCommand::Wallet => WalletCommand::uninstall_wallet().await?,
        }
        Ok(())
    }
}
