use anyhow::Result;
use std::process::Stdio;
use tokio::process::Command;

pub async fn async_command(command: &str) -> Result<String> {
    let child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .spawn()?
        .wait_with_output()
        .await?;
    Ok(String::from(String::from_utf8_lossy(&child.stdout)))
}

pub async fn async_command_pipe(command: &str) -> Result<String> {
    let output = Command::new("sh").arg("-c").arg(command).stdout(Stdio::piped()).output().await?;
    Ok(String::from(String::from_utf8_lossy(&output.stdout)))
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
