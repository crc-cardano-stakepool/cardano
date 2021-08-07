use structopt::StructOpt;

mod lib;
use lib::cardano::Command;

#[derive(Debug, StructOpt)]
#[structopt(name = "cardano", about = "A Cardano CLI built with Rust")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command
}


fn main() {
    let args = Cli::from_args();
    lib::cardano::start(args.cmd);
}
