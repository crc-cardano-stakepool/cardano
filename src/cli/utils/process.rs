use anyhow::{anyhow, Result};
use std::process::{Command as Cmd, Stdio};
use tokio::process::Command;

pub async fn async_command(command: &str) -> Result<String> {
    let child = Command::new("bash")
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

pub async fn async_command_pipe(command: &str) -> Result<String> {
    let process = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .await;
    match process {
        Ok(output) => Ok(String::from(String::from_utf8_lossy(&output.stdout))),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub async fn pipe(command: &str, pipe_command: &str) -> Result<String> {
    let mut child = Cmd::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    if let Some(output) = child.stdout.take() {
        let process = Cmd::new("sh")
            .arg("-c")
            .arg(pipe_command)
            .stdin(output)
            .stdout(Stdio::piped())
            .spawn()?;
        let process = process.wait_with_output();
        match process {
            Ok(output) => Ok(String::from(String::from_utf8_lossy(&output.stdout))),
            Err(e) => Err(anyhow!("{}", e)),
        }
    } else {
        Err(anyhow!("Failed executing piped command"))
    }
}

pub async fn process_success(cmd: &str) -> Result<bool> {
    let output = Command::new("sh").arg("-c").arg(&cmd).output().await?;
    if output.status.success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::utils::process::async_command_pipe;
    #[tokio::test]
    pub async fn test_async_command_pipe() {
        match async_command_pipe("find ./target/ -type f -name cardano | tail -n1").await {
            Ok(bin) => {
                let helper_string = "'{print $3}'";
                let cmd = format!("file {} | awk {}", bin.trim(), helper_string);
                match async_command_pipe(&cmd).await {
                    Ok(result) => assert_eq!("64-bit\n", result),
                    Err(e) => panic!("{}", e),
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
