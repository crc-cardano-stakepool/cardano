mod cli;
mod utils;
use cli::cli::cli_works;
use utils::terminal::async_command;

async fn main() {
    cli_works().expect("CLI crashed");
    async_command();
}
