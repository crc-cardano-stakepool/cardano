use crate::{async_command_pipe, print, SPINNERS};
use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Result as IOResult;

pub fn proceed(prompt: &str) -> IOResult<bool> {
    let color_theme = &ColorfulTheme::default();
    let dialog = String::from(prompt);
    Confirm::with_theme(color_theme).with_prompt(dialog).interact()
}

pub async fn spinner_cmd(cmd: &str, exec_msg: &'static str, finish_msg: &'static str) -> Result<()> {
    if let Some(arrows) = SPINNERS.get("arrows") {
        let spinner = ProgressBar::new_spinner();
        let spinner_style = ProgressStyle::default_spinner()
            .template("{msg} {spinner:.green}")
            .tick_strings(arrows);
        spinner.enable_steady_tick(80);
        spinner.set_style(spinner_style);
        spinner.set_message(exec_msg);
        async_command_pipe(cmd).await?;
        spinner.finish_and_clear();
        print("green", finish_msg)
    } else {
        Err(anyhow!("Failed executing command: {}", cmd))
    }
}

pub fn spinner(exec_msg: &'static str, tick_strings: &[&str]) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    let spinner_style = ProgressStyle::default_spinner()
        .template("{msg:.green} {spinner:.green}")
        .tick_strings(tick_strings);
    spinner.enable_steady_tick(80);
    spinner.set_style(spinner_style);
    spinner.set_message(exec_msg);
    spinner
}

#[cfg(test)]
mod test {
    // use super::*;
    //
    #[tokio::test]
    #[ignore]
    async fn test_spinner() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_spinner_cmd() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_proceed() {
        unimplemented!();
    }
}
