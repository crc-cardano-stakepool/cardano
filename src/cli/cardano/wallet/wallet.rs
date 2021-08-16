use crate::cli::utils::Terminal;
use anyhow::Result;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum WalletCommand {}

impl WalletCommand {
    pub async fn install_wallet() -> Result<()> {
        Terminal::print("white", "Installing cardano-wallet", Emoji("🤟", ""))?;
        Ok(())
    }

    pub async fn uninstall_wallet() -> Result<()> {
        Terminal::print("white", "Uninstalling cardano-wallet", Emoji("💔", ""))?;
        Ok(())
    }
}
