use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{CardanoComponent, Component};

#[derive(Debug, Args)]
pub struct AddressArgs {
    #[clap(subcommand)]
    command: AddressCommand,
}

#[derive(Debug, Subcommand)]
pub enum AddressCommand {
    /// Setup the system with cardano-address build dependencies
    Setup,
    /// Installs cardano-address
    Install {
        /// Confirm prompts automatically
        #[clap(short = 'y', long = "yes", value_parser, action)]
        confirm: bool,
    },
    /// Uninstalls cardano-address
    Uninstall,
}

impl AddressCommand {
    pub fn exec(cmd: AddressArgs) -> Result<()> {
        match cmd.command {
            AddressCommand::Install { confirm } => {
                CardanoComponent::check_latest_component(
                    Component::Address,
                    confirm,
                )
            }
            AddressCommand::Uninstall => {
                CardanoComponent::uninstall_component(Component::Address)
            }
            AddressCommand::Setup => {
                CardanoComponent::setup_component(Component::Address)
            }
        }
    }
}
