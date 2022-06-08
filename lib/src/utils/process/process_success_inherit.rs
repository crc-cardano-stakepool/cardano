use anyhow::{anyhow, Result};
use std::process::Stdio;
use tokio::process::Command;

pub async fn process_success_inherit(cmd: &str) -> Result<bool> {
    let child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::inherit())
        .spawn()?
        .wait_with_output()
        .await;
    match child {
        Ok(output) => Ok(output.status.success()),
        Err(e) => Err(anyhow!("{}", e)),
    }
}
