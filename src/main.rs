use structopt::StructOpt;
use cli::Cli;
mod cli;
mod utils;

#[tokio::main]
async fn main() {
    Cli::start(Cli::from_args().cmd).await
}
