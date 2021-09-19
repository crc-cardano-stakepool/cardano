use anyhow::Result;
use std::process::Stdio;
use tokio::process::Command;

pub async fn async_command(command: &str) -> Result<String> {
    let child = Command::new("sh")
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
        let res = async_command_pipe("file target/release/cardano | awk '{print $2}' ").await;
        match res {
            Ok(res) => assert_eq!("ELF\n", res),
            Err(e) => panic!("{}", e),
        }
    }
}
