use super::run::RunCommand;
use crate::cli::utils::terminal::Terminal;
use anyhow::Result;
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

    pub async fn check_node_version() -> Result<bool> {
        Terminal::print("white", "Checking cardano-node installation", Emoji("", ""));
        if let Ok(()) = Terminal::async_command("white", "cardano-node --version", Emoji("", "")).await {
            Terminal::print("green", "Cardano node is installed", Emoji("", ""));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn install_node() {
        if let Ok(true) = NodeCommand::check_node_version().await {
            if let Ok(true) = Terminal::proceed("Do you want to install the latest cardano node?") {
                Terminal::print("white", "Installing latest cardano node", Emoji("", ""));
                loop {
                    println!("Stop me if you can!")
                }
            } else {
                Terminal::print("red", "Aborted cardano-node installation", Emoji("", ""));
            }
        }
    }
}
