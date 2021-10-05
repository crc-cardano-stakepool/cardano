use anyhow::Result;
use console::Emoji;
use lib::{install_component, print_emoji};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage cardano wallets")]
pub enum WalletCommand {
    #[structopt(about = "Install the latest cardano-wallet binary")]
    Install {
        #[structopt(short = "y", long = "yes", help = "Confirm prompts automatically")]
        confirm: bool,
    },
    #[structopt(about = "Uninstalls the cardano-wallet binary")]
    Uninstall,
}

impl WalletCommand {
    pub async fn exec(cmd: WalletCommand) -> Result<()> {
        match cmd {
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
