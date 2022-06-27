use anyhow::Result;
use clap::{Args, Subcommand};
use console::Emoji;
use lib::{install_component, print_emoji};

#[derive(Debug, Args)]
pub struct WalletArgs {
    #[clap(subcommand)]
    command: WalletCommand,
}

#[derive(Debug, Subcommand)]
pub enum WalletCommand {
    Install { confirm: bool },
    Uninstall,
}

impl WalletCommand {
    pub async fn exec(cmd: WalletArgs) -> Result<()> {
        match cmd.command {
            WalletCommand::Install { confirm } => WalletCommand::install_wallet(confirm).await,
            WalletCommand::Uninstall => WalletCommand::uninstall_wallet().await,
        }
    }

    pub async fn install_wallet(confirm: bool) -> Result<()> {
        install_component("cardano-wallet", confirm).await
    }

    pub async fn uninstall_wallet() -> Result<()> {
        print_emoji("white", "Uninstalling cardano-wallet", Emoji("💔", ""))
    }
}
