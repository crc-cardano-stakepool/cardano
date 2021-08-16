use crate::cli::utils::print;
use anyhow::Result;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CliCommand {}

impl CliCommand {
    pub fn install_cli() -> Result<()> {
        print("", "Installing the latest cardano-cli", Emoji("🤟", ""))?;
        Ok(())
    }

    pub fn uninstall_cli() -> Result<()> {
        print("white", "Uninstalling cardano-cli", Emoji("💔", ""))?;
        Ok(())
    }
}
