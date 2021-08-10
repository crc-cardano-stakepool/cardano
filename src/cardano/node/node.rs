use super::run::RunCommand;
use crate::utils::command;
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
        println!("Checking for existing cardano-node binary");
        command("cardano-node --version").await;
    }
}
