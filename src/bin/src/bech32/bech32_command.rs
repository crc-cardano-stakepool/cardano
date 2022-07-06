use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{check_latest_bech32, setup_bech32, uninstall_bech32};

#[derive(Debug, Args)]
pub struct Bech32Args {
    #[clap(subcommand)]
    command: Bech32Command,
}

#[derive(Debug, Subcommand)]
pub enum Bech32Command {
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

impl Bech32Command {
    pub async fn exec(cmd: Bech32Args) -> Result<()> {
        match cmd.command {
            Bech32Command::Install { confirm } => {
                check_latest_bech32(confirm).await
            }
            Bech32Command::Uninstall => uninstall_bech32().await,
            Bech32Command::Setup => setup_bech32().await,
        }
    }
}
