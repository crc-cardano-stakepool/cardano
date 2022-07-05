use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{check_latest_wallet, setup_wallet, uninstall_wallet};

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
            WalletCommand::Install { confirm } => check_latest_wallet(confirm).await,
            WalletCommand::Uninstall => uninstall_wallet().await,
            WalletCommand::Setup => setup_wallet().await,
        }
    }
}
