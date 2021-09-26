use anyhow::Result;
use cardano_lib::print_emoji;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum WalletCommand {}

impl WalletCommand {
    pub async fn install_wallet() -> Result<()> {
        print_emoji("white", "Installing cardano-wallet", Emoji("🤟", ""))?;
        Ok(())
    }

    pub async fn uninstall_wallet() -> Result<()> {
        print_emoji("white", "Uninstalling cardano-wallet", Emoji("💔", ""))?;
        Ok(())
    }
}
