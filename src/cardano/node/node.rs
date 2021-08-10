use structopt::StructOpt;
use super::run::RunCommand;
use crate::utils::command;

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage cardano nodes")]
pub enum NodeCommand {
    Run(RunCommand),
}

impl NodeCommand {
    pub fn exec(cmd: NodeCommand) {
        match cmd {
            NodeCommand::Run(cmd) => RunCommand::exec(cmd),
        }
    }

    pub fn check_node_version() {
        println!("Checking for existing cardano-node binary");
        command("cardano-node --version");
    }
}
