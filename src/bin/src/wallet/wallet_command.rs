use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{check_latest_version, install_wallet, proceed, set_confirm, setup_wallet, uninstall_wallet, Component};

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
    Install {
        /// Confirm prompts automatically
        #[clap(short = 'y', long = "yes", value_parser, action)]
        confirm: bool,
    },
    /// Uninstalls cardano-wallet
    Uninstall,
}

impl WalletCommand {
    pub async fn exec(cmd: WalletArgs) -> Result<()> {
        match cmd.command {
            WalletCommand::Install { confirm } => {
                set_confirm(confirm);
                let version = check_latest_version(Component::Wallet).await?;
                let msg = format!("Do you want to install the latest cardano-wallet {version} binary?");
                if !confirm && proceed(&msg)? {
                    return install_wallet().await;
                }
                setup_wallet().await?;
                install_wallet().await
            }
            WalletCommand::Uninstall => uninstall_wallet().await,
            WalletCommand::Setup => setup_wallet().await,
        }
    }
}
