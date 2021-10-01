use crate::{async_command_pipe, print, SPINNERS};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn spinner_cmd(
    cmd: &str,
    exec_msg: &'static str,
    finish_msg: &'static str,
) -> Result<()> {
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
        print("green", finish_msg)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::spinner_cmd;
    #[tokio::test]
    #[ignore]
    async fn test_spinner_cmd() {
        unimplemented!();
    }
}
