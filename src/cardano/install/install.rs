use crate::cardano::component::CardanoComponent;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Install cardano components")]
pub enum InstallCommand {
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
        println!("Installing cardano-cli")
    }

    fn install_node() {
        println!("Installing cardano-node")
    }

    fn install_wallet() {
        println!("Installing cardano-wallet")
    }

    fn install_db() {
        println!("Installing cardano-db-sync")
    }
}
