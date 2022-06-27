extern crate lib;
pub mod cli;
use anyhow::Result;
use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator};
pub use cli::*;
use ctrlc::set_handler;
use human_panic::setup_panic;

#[tokio::main]
async fn main() -> Result<()> {
    setup_panic!();
    set_handler(|| println!("Initialize Ctrl-C handler")).expect("Error setting Ctrl-C handler");
    let cli = Cli::parse();
    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        eprintln!("Generating completion file for {:?}...", generator);
        print_completions(generator, &mut cmd);
        Ok(())
    } else if let Some(command) = cli.command {
        Cli::start(command).await
    } else {
        Ok(())
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
