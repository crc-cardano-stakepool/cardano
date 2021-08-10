use super::run::*;
use structopt::StructOpt;

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
}
