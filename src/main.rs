use cli::Cli;
use ctrlc::set_handler;
use structopt::StructOpt;
mod cli;

#[tokio::main]
async fn main() {
    Cli::start(Cli::from_args().command).await;
    set_handler(|| println!("Initialize Ctrl-C handler")).expect("Error setting Ctrl-C handler");
}
