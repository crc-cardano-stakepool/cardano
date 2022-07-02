use crate::check_user;
use anyhow::{anyhow, Result};
use std::process::{Command as Cmd, Stdio};
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
        Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
        Err(e) => Err(anyhow!("{e}")),
    }
}

pub async fn async_command_pipe(command: &str) -> Result<String> {
    log::info!("Executing command: {command}");
    let process = Command::new("sh").arg("-c").arg(command).stdout(Stdio::piped()).output().await;
    match process {
        Ok(output) => {
            let output = String::from(String::from_utf8_lossy(&output.stdout)).trim().to_string();
            log::debug!("Output: {output}");
            Ok(output)
        }
        Err(e) => {
            log::error!("Command failed");
            Err(anyhow!("{e}"))
        }
    }
}

pub async fn async_user_command(command: &str) -> Result<()> {
    let user = check_user()?;
    let cmd = format!("sudo su - {user} -c \"eval {command}\"");
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
            .spawn()?
            .wait_with_output();
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

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_async_command() -> Result<()> {
        let output = async_command("echo 'expected to be printed on console' >/dev/null").await?;
        assert_eq!(output, "");
        Ok(())
    }

    #[tokio::test]
    pub async fn test_async_command_pipe() -> Result<()> {
        let expected = "not expected to be printed on console";
        let cmd = format!("echo {expected}");
        let output = async_command_pipe(&cmd).await?;
        assert_eq!(output, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_program_installed() -> Result<()> {
        let result = is_program_installed("totally_not_an_installed_program").await?;
        assert!(!result);
        let result = is_program_installed("ls").await?;
        assert!(result);
        Ok(())
    }

    #[tokio::test]
    async fn test_process_success() -> Result<()> {
        let result = process_success("true").await?;
        assert!(result);
        let result = process_success("false").await?;
        assert!(!result);
        Ok(())
    }

    #[tokio::test]
    async fn test_process_success_inherit() -> Result<()> {
        let expected = "expected";
        let cmd = format!("echo {expected} >/dev/null");
        let result = process_success_inherit(&cmd).await?;
        assert!(result);
        let cmd = format!("echo {expected} >/dev/null && false");
        let result = process_success_inherit(&cmd).await?;
        assert!(!result);
        Ok(())
    }

    #[tokio::test]
    async fn test_pipe() -> Result<()> {
        let output = pipe("echo test", "grep test").await?;
        assert_eq!(output, "test\n");
        let output = pipe("echo test", "grep fails").await?;
        assert_ne!(output, "test\n");
        Ok(())
    }
}
