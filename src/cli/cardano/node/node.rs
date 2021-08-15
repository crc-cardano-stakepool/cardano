use super::run::RunCommand;
use crate::cli::utils::terminal::Terminal;
use anyhow::Result;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage cardano nodes")]
pub enum NodeCommand {
    Run(RunCommand),
    #[structopt(about = "Install the latest cardano-node binary")]
    Install,
}

impl NodeCommand {
    pub async fn exec(cmd: NodeCommand) -> Result<()> {
        match cmd {
            NodeCommand::Run(cmd) => RunCommand::exec(cmd).await?,
            NodeCommand::Install => NodeCommand::install_node().await?,
        }
        Ok(())
    }

    pub async fn check_node_version() -> Result<bool> {
        Terminal::print("white", "Checking cardano-node installation", Emoji("", ""))?;
        if let Ok(_) = Terminal::async_command("white", "cardano-node --version", Emoji("", "")).await {
            Terminal::print("green", "Cardano node is installed", Emoji("", ""))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn install_node() -> Result<()> {
        if let Ok(true) = NodeCommand::check_node_version().await {
            if let Ok(true) = Terminal::proceed("Do you want to install the latest cardano node?") {
                Terminal::print("white", "Installing latest cardano node", Emoji("", ""))?;
            } else {
                Terminal::print("red", "Aborted cardano-node installation", Emoji("", ""))?;
            }
        }
        Ok(())
    }
}
