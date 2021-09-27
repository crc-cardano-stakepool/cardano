use crate::{async_command_pipe, print};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn spinner_cmd(cmd: &str, exec_msg: &'static str, finish_msg: &'static str) -> Result<()> {
    let spinner = ProgressBar::new_spinner();
    let spinner_style = ProgressStyle::default_spinner()
        .template("{msg} {spinner:.green}")
        .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"]);
    spinner.enable_steady_tick(80);
    spinner.set_style(spinner_style);
    spinner.set_message(exec_msg);
    async_command_pipe(cmd).await?;
    spinner.finish_and_clear();
    print("green", finish_msg)?;
    Ok(())
}
