use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{install_wallet, proceed, set_confirm, setup_wallet, uninstall_wallet};

#[derive(Debug, Args)]
pub struct WalletArgs {
    #[clap(subcommand)]
    command: WalletCommand,
}

#[derive(Debug, Subcommand)]
pub enum WalletCommand {
    /// Setup the system with cardano-wallet build dependencies
    Setup,
    /// Installs cardano-wallet
    Install { confirm: bool },
    /// Uninstalls cardano-wallet
    Uninstall,
}

impl WalletCommand {
    pub async fn exec(cmd: WalletArgs) -> Result<()> {
        match cmd.command {
            WalletCommand::Install { confirm } => {
                set_confirm(confirm);
                setup_wallet().await?;
                if !confirm && proceed("Do you want to install the latest cardano-wallet binary?")? {
                    return install_wallet().await;
                }
                install_wallet().await
            }
            WalletCommand::Uninstall => uninstall_wallet().await,
            WalletCommand::Setup => setup_wallet().await,
        }
    }
}
