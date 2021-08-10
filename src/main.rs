use structopt::StructOpt;
use cli::*;
mod cli;
mod cardano;
mod utils;

fn main() {
    Cli::start(Cli::from_args().cmd)
}
