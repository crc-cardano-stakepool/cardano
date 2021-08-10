use cli::*;
use structopt::StructOpt;
mod cardano;
mod cli;
mod utils;

#[tokio::main]
async fn main() {
    Cli::start(Cli::from_args().cmd).await
}
