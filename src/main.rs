use structopt::StructOpt;
use cli::*;
mod cli;
mod cardano;

fn main() {
    Cli::start(Cli::from_args().cmd)
}
