use structopt::StructOpt;
use crate::cardano::component::CardanoComponent;
use crate::utils::command;
use super::super::node::NodeCommand;

#[derive(Debug, StructOpt)]
#[structopt(about = "Install cardano components")]
pub enum InstallCommand {
    #[structopt(about = "Installs a cardano component")]
    Component(CardanoComponent),
}

impl InstallCommand {
    pub fn exec(cmd: InstallCommand) {
        match cmd {
            InstallCommand::Component(comp) => InstallCommand::install_component(comp),
        }
    }

    pub fn install_component(comp: CardanoComponent) {
        match comp {
            CardanoComponent::Node => InstallCommand::install_node(),
            CardanoComponent::Cli => InstallCommand::install_cli(),
            CardanoComponent::Wallet => InstallCommand::install_wallet(),
            CardanoComponent::Db => InstallCommand::install_db(),
        };
    }

    fn install_cli() {
        println!("Checking cardano-cli installation");
        command("cardano-cli --version");
        println!("Installing cardano-node");
    }

    fn install_node() {
        println!("Checking cardano-node installation");
        NodeCommand::check_node_version();
        println!("Installing cardano-node");
        println!("Installing cardano-node");
    }

    fn install_wallet() {
        println!("Checking cardano-wallet installation");
        command("cardano-wallet --version");
        println!("Installing cardano-wallet");
    }

    fn install_db() {
        println!("Checking cardano-db-sync installation");
        command("cardano-db-sync --version");
        println!("Installing cardano-db-sync");
    }
}
