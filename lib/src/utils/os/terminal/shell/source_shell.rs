use crate::{async_command_pipe, check_env};
use anyhow::Result;

pub async fn source_shell() -> Result<()> {
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    let cmd = format!("source {}", shell_profile_file);
    async_command_pipe(&cmd).await?;
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
