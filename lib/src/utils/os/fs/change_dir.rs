use crate::{async_command, print};
use anyhow::Result;

pub async fn change_dir(absolute_path: &str) -> Result<()> {
    let msg = format!("Changing directory to {}", absolute_path);
    let cmd = format!("cd {}", absolute_path);
    print("", &msg)?;
    async_command(&cmd).await?;
    Ok(())
}
