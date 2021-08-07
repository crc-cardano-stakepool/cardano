use structopt::StructOpt;
mod cardano;
use crate::cardano::*;

fn main() {
    Cli::start(Cli::from_args().cmd);
}
