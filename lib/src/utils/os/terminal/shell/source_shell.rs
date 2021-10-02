use crate::{async_command_pipe, get_shell_profile_file};
use anyhow::Result;

pub async fn source_shell() -> Result<String> {
    let shell_file = get_shell_profile_file().await?;
    let cmd = format!("source {}", shell_file);
    async_command_pipe(&cmd).await
}
