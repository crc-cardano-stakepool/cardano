use crate::{Run, RunCommand};
use anyhow::Result;
use clap::{Args, Subcommand};
use console::Emoji;
use lib::{install_component, print_emoji, Component};

#[derive(Debug, Args)]
pub struct NodeArgs {
    #[clap(subcommand)]
    command: NodeCommand,
}

#[derive(Debug, Subcommand)]
pub enum NodeCommand {
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
            NodeCommand::Install { confirm } => NodeCommand::install_node(confirm).await,
            NodeCommand::Uninstall => NodeCommand::uninstall_node().await,
        }
    }

    pub async fn install_node(confirm: bool) -> Result<()> {
        install_component(Component::Node, confirm).await
    }

    pub async fn uninstall_node() -> Result<()> {
        print_emoji("white", "Uninstalling cardano-node", Emoji("💔", ""))
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn test_install_node() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_uninstall_node() {
        unimplemented!();
    }
}
