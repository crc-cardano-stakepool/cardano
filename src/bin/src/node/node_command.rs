use crate::{Run, RunCommand};
use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{CardanoComponent, Component};

#[derive(Debug, Args)]
pub struct NodeArgs {
    #[clap(subcommand)]
    command: NodeCommand,
}

#[derive(Debug, Subcommand)]
pub enum NodeCommand {
    /// Setup the system with cardano build dependencies
    Setup,
    /// Run cardano-node
    Run(Run),
    /// Install the latest cardano-node binary
    Install {
        /// Confirm prompts automatically
        #[clap(short = 'y', long = "yes", value_parser, action)]
        confirm: bool,
    },
    /// Uninstall cardano-node
    Uninstall,
}

impl NodeCommand {
    pub fn exec(cmd: NodeArgs) -> Result<()> {
        match cmd.command {
            NodeCommand::Run(cmd) => RunCommand::exec(cmd),
            NodeCommand::Install { confirm } => {
                CardanoComponent::check_latest_component(
                    Component::Node,
                    confirm,
                )
            }
            NodeCommand::Uninstall => {
                CardanoComponent::uninstall_component(Component::Node)
            }
            NodeCommand::Setup => {
                CardanoComponent::setup_component(Component::Node)
            }
        }
    }
}
