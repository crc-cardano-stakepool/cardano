use cli::*;
use structopt::StructOpt;
mod cardano;
mod cli;
mod utils;

fn main() {
    Cli::start(Cli::from_args().cmd)
}
