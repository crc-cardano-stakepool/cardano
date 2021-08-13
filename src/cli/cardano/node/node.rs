use super::run::RunCommand;
use crate::cli::utils::proceed;
use crate::cli::utils::terminal::Terminal;
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
        Terminal::print("white", "Checking cardano-node installation", Emoji("❕", ""))
            .await
            .expect("Failed printing to terminal");
        Terminal::async_command("green", "cardano-node --version", Emoji("🤌", ""))
            .await
            .expect("Failed executing command");
    }

    pub async fn install_node() {
        Terminal::print("white", "Installing latest cardano node", Emoji("", ""))
            .await
            .expect("Failed printing to terminal");
        proceed().expect("Aborting node update");
    }
}
