use crate::cli::utils::Terminal;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CliCommand {}

impl CliCommand {
    pub fn install_cli() {
        Terminal::print("white", "Checking cardano-cli installation", Emoji("❕", ""));
        Terminal::print("white", "Installing cardano-cli", Emoji("❕", ""));
        Terminal::print("white", "Installing cardano-cli", Emoji("❕", ""));
    }
}
