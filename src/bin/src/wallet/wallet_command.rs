use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{
    check_latest_component, setup_component, uninstall_component, Component,
};

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
    pub fn exec(cmd: WalletArgs) -> Result<()> {
        match cmd.command {
            WalletCommand::Install { confirm } => {
                check_latest_component(Component::Wallet, confirm)
            }
            WalletCommand::Uninstall => uninstall_component(Component::Wallet),
            WalletCommand::Setup => setup_component(Component::Wallet),
        }
    }
}
