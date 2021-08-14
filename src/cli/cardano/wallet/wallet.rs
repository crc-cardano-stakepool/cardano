use crate::cli::utils::Terminal;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum WalletCommand {}

impl WalletCommand {
    pub async fn install_wallet() {
        Terminal::print("white", "Checking cardano-wallet installation", Emoji("❕", ""));
        Terminal::print("white", "Installing cardano-wallet", Emoji("❕", ""));
    }
}
