use crate::{Run, RunCommand};
use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{check_latest_version, install_node, install_wallet, proceed, set_confirm, setup_node, uninstall_component, Component};

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
                let version = check_latest_version(Component::Node).await?;
                let msg = format!("Do you want to install the latest cardano-node v{version} binary?");
                if !confirm && proceed(&msg)? {
                    return install_wallet().await;
                }
                setup_node().await?;
                install_node().await
            }
            NodeCommand::Uninstall => uninstall_component("cardano-node").await,
            NodeCommand::Setup => setup_node().await,
        }
    }
}
