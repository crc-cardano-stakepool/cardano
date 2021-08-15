use crate::cli::utils::Terminal;
use anyhow::Result;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CliCommand {}

impl CliCommand {
    pub fn install_cli() -> Result<()> {
        Terminal::print("white", "Checking cardano-cli installation", Emoji("❕", ""))?;
        Terminal::print("white", "Installing cardano-cli", Emoji("❕", ""))?;
        Terminal::print("white", "Installing cardano-cli", Emoji("❕", ""))?;
        Ok(())
    }
}
