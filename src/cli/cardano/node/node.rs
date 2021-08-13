use super::run::RunCommand;
use crate::cli::utils::terminal::Terminal;
use crate::cli::utils::types::TResult;
use console::Emoji;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage cardano nodes")]
pub enum NodeCommand {
    Run(RunCommand),
}

impl NodeCommand {
    pub async fn exec(cmd: NodeCommand) {
        match cmd {
            NodeCommand::Run(cmd) => RunCommand::exec(cmd).await,
        }
    }

    pub async fn check_node_version() -> TResult<bool> {
        Terminal::print("white", "Checking cardano-node installation", Emoji("", "")).await;
        if let Ok(()) = Terminal::async_command("white", "cardano-node --version", Emoji("", "")).await {
            Terminal::print("green", "Cardano node is installed", Emoji("", "")).await;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn install_node() {
        if let Ok(true) = NodeCommand::check_node_version().await {
            if let Ok(true) = Terminal::proceed("Do you want to install the latest cardano node?") {
                Terminal::print("white", "Installing latest cardano node", Emoji("", "")).await;
            } else {
                Terminal::print("red", "Aborted cardano-node installation", Emoji("", "")).await;
            }
        }
    }
}
