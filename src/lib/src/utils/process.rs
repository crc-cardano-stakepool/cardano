use anyhow::{anyhow, Result};
use std::process::{Command, Stdio};

pub fn async_command(command: &str) -> Result<String> {
    log::debug!("Executing command: {command}");
    let child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .spawn()?
        .wait_with_output();
    match child {
        Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
        Err(e) => Err(anyhow!("{e}")),
    }
}

pub fn async_command_pipe(command: &str) -> Result<String> {
    log::debug!("Executing command: {command}");
    let process = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .output();
    match process {
        Ok(output) => Ok(String::from(String::from_utf8_lossy(&output.stdout))
            .trim()
            .to_string()),
        Err(e) => {
            log::error!("Command failed");
            Err(anyhow!("{e}"))
        }
    }
}

pub fn is_program_installed(program: &str) -> Result<bool> {
    let cmd = format!("type {program}");
    process_success(&cmd)
}

pub fn pipe(command: &str, pipe_command: &str) -> Result<String> {
    log::debug!("Executing command: {command} | {pipe_command}");
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    if let Some(output) = child.stdout.take() {
        let process = Command::new("sh")
            .arg("-c")
            .arg(pipe_command)
            .stdin(output)
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output();
        match process {
            Ok(output) => {
                Ok(String::from(String::from_utf8_lossy(&output.stdout)))
            }
            Err(e) => Err(anyhow!("{e}")),
        }
    } else {
        Err(anyhow!("Failed executing piped command"))
    }
}

pub fn process_success_inherit(cmd: &str) -> Result<bool> {
    log::debug!("Executing command: {cmd}");
    let child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::inherit())
        .spawn()?
        .wait_with_output();
    match child {
        Ok(output) => Ok(output.status.success()),
        Err(e) => Err(anyhow!("{e}")),
    }
}

pub fn process_success(cmd: &str) -> Result<bool> {
    log::debug!("Checking for success of command: {cmd}");
    let output = Command::new("sh").arg("-c").arg(&cmd).output()?;
    Ok(output.status.success())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_async_command() -> Result<()> {
        let output = async_command(
            "echo 'expected to be printed on console' >/dev/null",
        )?;
        assert_eq!(output, "");
        Ok(())
    }

    #[test]
    pub fn test_async_command_pipe() -> Result<()> {
        let expected = "not expected to be printed on console";
        let cmd = format!("echo {expected}");
        let output = async_command_pipe(&cmd)?;
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_is_program_installed() -> Result<()> {
        let result = is_program_installed("totally_not_an_installed_program")?;
        assert!(!result);
        let result = is_program_installed("ls")?;
        assert!(result);
        Ok(())
    }

    #[test]
    fn test_process_success() -> Result<()> {
        let result = process_success("true")?;
        assert!(result);
        let result = process_success("false")?;
        assert!(!result);
        Ok(())
    }

    #[test]
    fn test_process_success_inherit() -> Result<()> {
        let expected = "expected";
        let cmd = format!("echo {expected} >/dev/null");
        let result = process_success_inherit(&cmd)?;
        assert!(result);
        let cmd = format!("echo {expected} >/dev/null && false");
        let result = process_success_inherit(&cmd)?;
        assert!(!result);
        Ok(())
    }

    #[test]
    fn test_pipe() -> Result<()> {
        let output = pipe("echo test", "grep test")?;
        assert_eq!(output, "test\n");
        let output = pipe("echo test", "grep fails")?;
        assert_ne!(output, "test\n");
        Ok(())
    }
}
