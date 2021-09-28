use anyhow::Result;
use tokio::process::Command;

pub async fn process_success(cmd: &str) -> Result<bool> {
    let output = Command::new("sh").arg("-c").arg(&cmd).output().await?;
    if output.status.success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    // use crate::process_success;
    #[tokio::test]
    #[ignore]
    async fn test_process_success() {
        unimplemented!();
    }
}
