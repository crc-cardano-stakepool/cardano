extern crate lib;
pub mod cli;
use anyhow::Result;
pub use cli::*;
use ctrlc::set_handler;
use human_panic::setup_panic;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    setup_panic!();
    set_handler(|| println!("Initialize Ctrl-C handler")).expect("Error setting Ctrl-C handler");
    Cli::start(Cli::from_args().command).await
}

