use super::super::node::NodeCommand;
use crate::cli::component::CardanoComponent;
use crate::utils::Terminal;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Install cardano components")]
pub enum InstallCommand {
    #[structopt(about = "Installs a cardano component")]
    Component(CardanoComponent),
}

impl InstallCommand {
    pub async fn exec(cmd: InstallCommand) {
        match cmd {
            InstallCommand::Component(comp) => InstallCommand::install_component(comp).await,
        }
    }

    pub async fn install_component(comp: CardanoComponent) {
        match comp {
            CardanoComponent::Node => InstallCommand::install_node().await,
            CardanoComponent::Cli => InstallCommand::install_cli().await,
            CardanoComponent::Wallet => InstallCommand::install_wallet().await,
            CardanoComponent::Db => InstallCommand::install_db().await,
        };
    }

    async fn install_node() {
        NodeCommand::check_node_version().await;
        Terminal::print("green", "Installing cardano-node", Emoji("✅", ""));
    }

    async fn install_cli() {
        Terminal::print("white", "Checking cardano-cli installation", Emoji("❕", ""));
        Terminal::async_command("white", "cardano-cli --version", Emoji("❕", "")).await;
        Terminal::print("white", "Installing cardano-cli", Emoji("❕", ""));
    }

    async fn install_wallet() {
        Terminal::print("white", "Checking cardano-wallet installation", Emoji("❕", ""));
        Terminal::async_command("white", "cardano-wallet --version", Emoji("", "")).await;
        Terminal::print("white", "Installing cardano-wallet", Emoji("❕", ""));
    }

    async fn install_db() {
        Terminal::print("white", "Checking cardano-db-sync installation", Emoji("❕", ""));
        Terminal::async_command("white", "cardano-db-sync --version", Emoji("", "")).await;
        Terminal::print("white", "Installing cardano-db-sync", Emoji("❕", ""));
    }
}
