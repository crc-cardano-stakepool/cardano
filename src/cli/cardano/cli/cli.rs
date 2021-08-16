use crate::cli::utils::Terminal;
use anyhow::Result;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CliCommand {}

impl CliCommand {
    pub fn install_cli() -> Result<()> {
        Terminal::print("", "Installing the latest cardano-cli", Emoji("🤟", ""))?;
        Ok(())
    }
    
    pub fn uninstall_cli() -> Result<()> {
        Terminal::print("white", "Uninstalling cardano-cli", Emoji("💔", ""))?;
        Ok(())
    }
}
