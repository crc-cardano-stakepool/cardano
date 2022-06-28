use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{install_component, uninstall_component};

#[derive(Debug, Args)]
pub struct WalletArgs {
    #[clap(subcommand)]
    command: WalletCommand,
}

#[derive(Debug, Subcommand)]
pub enum WalletCommand {
    /// Installs cardano-wallet
    Install { confirm: bool },
    /// Uninstalls cardano-wallet
    Uninstall,
}

impl WalletCommand {
    pub async fn exec(cmd: WalletArgs) -> Result<()> {
        match cmd.command {
            WalletCommand::Install { confirm } => install_component("cardano-wallet", confirm).await,
            WalletCommand::Uninstall => uninstall_component("cardano-wallet").await,
        }
    }
}
