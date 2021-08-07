use structopt::StructOpt;
mod cardano;
use crate::cardano::Cli;

fn main() {
    Cli::start(Cli::from_args().cmd);
}
