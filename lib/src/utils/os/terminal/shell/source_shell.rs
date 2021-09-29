use crate::async_command;
use anyhow::Result;

pub async fn source_shell(shell_profile_file: &str) -> Result<()> {
    let cmd = format!("source {}", shell_profile_file);
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::source_shell;
    #[tokio::test]
    #[ignore]
    async fn test_source_shell() {
        unimplemented!();
    }
}
