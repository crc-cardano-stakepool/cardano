use anyhow::{anyhow, Result};
use std::process::{Command, Stdio};

pub struct Executer;

impl Executer {
    pub fn exec(command: &str) -> Result<()> {
        log::debug!("Executing command: {command}");
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::inherit())
            .spawn()?
            .wait_with_output()
            .map(|_| ())
            .map_err(|err| anyhow!("Failed to execute command: {err}"))
    }

    pub fn capture(command: &str) -> Result<String> {
        log::debug!("Executing command: {command}");
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .output()
            .map(|output| {
                String::from(String::from_utf8_lossy(&output.stdout))
                    .trim()
                    .to_string()
            })
            .map_err(|err| anyhow!("Failed to execute command: {err}"))
    }

    pub fn is_program_installed(program: &str) -> Result<bool> {
        let cmd = format!("type {program}");
        Self::success(&cmd)
    }

    pub fn pipe(command: &str, pipe_command: &str) -> Result<String> {
        log::debug!("Executing command: {command} | {pipe_command}");
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|err| anyhow!("Failed to spawn stdout: {err}"))
            .unwrap()
            .stdout
            .take()
            .ok_or("Failed to take child stdout")
            .map(|output| {
                return Command::new("sh")
                    .arg("-c")
                    .arg(pipe_command)
                    .stdin(output)
                    .stdout(Stdio::piped())
                    .spawn()
                    .map_err(|err| anyhow!("Failed to spawn stdout: {err}"))
                    .unwrap()
                    .wait_with_output()
                    .map(|output| {
                        return String::from(String::from_utf8_lossy(
                            &output.stdout,
                        ));
                    })
                    .map_err(|err| anyhow!("Failed to execute command: {err}"))
                    .unwrap();
            })
            .map_err(|err| anyhow!("Failed executing piped command: {err}"))
    }

    pub fn capture_success(cmd: &str) -> Result<bool> {
        log::debug!("Executing command: {cmd}");
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::inherit())
            .spawn()
            .map_err(|err| anyhow!("Failed to spawn stdout: {err}"))
            .unwrap()
            .wait_with_output()
            .map(|output| output.status.success())
            .map_err(|err| anyhow!("Failed to execute command: {err}"))
    }

    pub fn success(cmd: &str) -> Result<bool> {
        log::debug!("Checking for success of command: {cmd}");
        Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .map(|output| output.status.success())
            .map_err(|err| anyhow!("Failed to execute command: {err}"))
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_async_command() -> Result<()> {
        let output = Executer::exec(
            "echo 'expected to be printed on console' >/dev/null",
        )?;
        assert_eq!(output, ());
        Ok(())
    }

    #[test]
    pub fn test_async_command_pipe() -> Result<()> {
        let expected = "not expected to be printed on console";
        let cmd = format!("echo {expected}");
        let output = Executer::capture(&cmd)?;
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_is_program_installed() -> Result<()> {
        let result =
            Executer::is_program_installed("totally_not_an_installed_program")?;
        assert!(!result);
        let result = Executer::is_program_installed("ls")?;
        assert!(result);
        Ok(())
    }

    #[test]
    fn test_process_success() -> Result<()> {
        let result = Executer::success("true")?;
        assert!(result);
        let result = Executer::success("false")?;
        assert!(!result);
        Ok(())
    }

    #[test]
    fn test_process_success_inherit() -> Result<()> {
        let expected = "expected";
        let cmd = format!("echo {expected} >/dev/null");
        let result = Executer::capture_success(&cmd)?;
        assert!(result);
        let cmd = format!("echo {expected} >/dev/null && false");
        let result = Executer::capture_success(&cmd)?;
        assert!(!result);
        Ok(())
    }

    #[test]
    fn test_pipe() -> Result<()> {
        let output = Executer::pipe("echo test", "grep test")?;
        assert_eq!(output, "test\n");
        let output = Executer::pipe("echo test", "grep fails")?;
        assert_ne!(output, "test\n");
        Ok(())
    }
}
