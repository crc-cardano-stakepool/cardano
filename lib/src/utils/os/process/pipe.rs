use anyhow::{anyhow, Result};
use std::process::{Command as Cmd, Stdio};

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

#[cfg(test)]
mod test {
    // use crate::pipe;
    #[tokio::test]
    #[ignore]
    async fn test_pipe() {
        unimplemented!();
    }
}
