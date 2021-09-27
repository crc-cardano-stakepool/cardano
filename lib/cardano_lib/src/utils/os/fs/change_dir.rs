use crate::async_command;
use anyhow::Result;

pub async fn change_dir(absolute_path: &str) -> Result<()> {
    let cmd = format!("cd {}", absolute_path);
    async_command(&cmd).await?;
    Ok(())
}
