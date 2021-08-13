use crate::cli::utils::Terminal;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum WalletCommand {}

impl WalletCommand {
    pub async fn install_wallet() {
        Terminal::print("white", "Checking cardano-wallet installation", Emoji("❕", ""))
            .await
            .expect("Failed printing to terminal");
        Terminal::async_command("white", "cardano-wallet --version", Emoji("", ""))
            .await
            .expect("Failed printing to terminal");
        Terminal::print("white", "Installing cardano-wallet", Emoji("❕", ""))
            .await
            .expect("Failed printing to terminal");
    }
}
