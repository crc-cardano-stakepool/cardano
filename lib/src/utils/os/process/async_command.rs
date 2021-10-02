use anyhow::{anyhow, Result};
use std::process::Stdio;
use tokio::process::Command;

pub async fn async_command(command: &str) -> Result<String> {
    let child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .spawn()?
        .wait_with_output()
        .await;
    match child {
        Ok(output) => Ok(String::from(String::from_utf8_lossy(&output.stdout))),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

#[cfg(test)]
mod test {
    // use crate::async_command;
    #[tokio::test]
    #[ignore]
    async fn test_async_command() {
        unimplemented!();
    }
}
