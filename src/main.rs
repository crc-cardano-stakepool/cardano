use cli::Cli;
use structopt::StructOpt;
mod cli;
mod utils;
mod tests;

#[tokio::main]
async fn main() {
    Cli::start(Cli::from_args().cmd).await
}
