use crate::check_user;
use anyhow::{anyhow, Result};
use std::process::{Command as Cmd, Stdio};
use tokio::process::Command;

pub async fn async_command_pipe(command: &str) -> Result<String> {
    let process = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .await;
    match process {
        Ok(output) => Ok(String::from(String::from_utf8_lossy(&output.stdout))),
        Err(e) => Err(anyhow!("{e}")),
    }
}

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
        Err(e) => Err(anyhow!("{e}")),
    }
}

pub async fn async_user_command(command: &str) -> Result<()> {
    let user = check_user().await?;
    let cmd = format!("su - {user} -c \"eval {command}\"");
    async_command(&cmd).await?;
    Ok(())
}

pub async fn is_program_installed(program: &str) -> Result<bool> {
    let cmd = format!("type {program}");
    process_success(&cmd).await
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
            Err(e) => Err(anyhow!("{e}")),
        }
    } else {
        Err(anyhow!("Failed executing piped command"))
    }
}

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
        Err(e) => Err(anyhow!("{e}")),
    }
}

pub async fn process_success(cmd: &str) -> Result<bool> {
    let output = Command::new("sh").arg("-c").arg(&cmd).output().await?;
    Ok(output.status.success())
}

pub async fn user_process_success(command: &str) -> Result<bool> {
    let user = check_user().await?;
    let cmd = format!("su - {user} -c \"eval {command}\"");
    let output = Command::new("sh").arg("-c").arg(&cmd).output().await?;
    Ok(output.status.success())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_process_success() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_pipe() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_install_ghc() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_async_command() {
        unimplemented!();
    }

    #[tokio::test]
    pub async fn test_async_command_pipe() {
        match async_command_pipe("find ../target/ -type f -name cardano | tail -n1").await {
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
