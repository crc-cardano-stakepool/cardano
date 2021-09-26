extern crate lib;
mod cli;
use anyhow::Result;
use cli::Cli;
use ctrlc::set_handler;
use human_panic::setup_panic;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    setup_panic!();
    Cli::start(Cli::from_args().command).await?;
    set_handler(|| println!("Initialize Ctrl-C handler")).expect("Error setting Ctrl-C handler");
    Ok(())
}
