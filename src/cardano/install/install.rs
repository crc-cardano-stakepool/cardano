use super::super::node::NodeCommand;
use crate::cardano::component::CardanoComponent;
use crate::utils::command;
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
        println!("Checking cardano-node installation");
        NodeCommand::check_node_version().await;
        println!("Installing cardano-node")
    }

    async fn install_cli() {
        println!("Checking cardano-cli installation");
        command("cardano-cli --version").await;
        println!("Installing cardano-node");
    }

    async fn install_wallet() {
        println!("Checking cardano-wallet installation");
        command("cardano-wallet --version").await;
        println!("Installing cardano-wallet");
    }

    async fn install_db() {
        println!("Checking cardano-db-sync installation");
        command("cardano-db-sync --version").await;
        println!("Installing cardano-db-sync");
    }
}
