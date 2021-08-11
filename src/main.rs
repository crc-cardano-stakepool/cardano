use cli::Cli;
use structopt::StructOpt;
mod cli;
mod tests;
mod utils;

#[tokio::main]
async fn main() {
    Cli::start(Cli::from_args().cmd).await
}
