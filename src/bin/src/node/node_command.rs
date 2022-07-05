use crate::{Run, RunCommand};
use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{install_node, set_confirm, setup_node, uninstall_component};

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
    pub async fn exec(cmd: NodeArgs) -> Result<()> {
        match cmd.command {
            NodeCommand::Run(cmd) => RunCommand::exec(cmd).await,
            NodeCommand::Install { confirm } => {
                set_confirm(confirm);
                setup_node().await?;
                install_node().await
            }
            NodeCommand::Uninstall => uninstall_component("cardano-node").await,
            NodeCommand::Setup => setup_node().await,
        }
    }
}
