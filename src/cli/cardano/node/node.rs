use super::run::RunCommand;
use crate::utils::terminal::*;
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

    pub async fn check_node_version() {
        Terminal::print("white", "Checking cardano-node installation", Emoji("❕", "")).await;
        Terminal::async_command("green", "cardano-node --version", Emoji("🤌", "")).await;
    }
}