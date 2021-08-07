use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum NodeCommand {
    Run(RunCommand),
}

#[derive(Debug, StructOpt)]
pub enum RunCommand {}
