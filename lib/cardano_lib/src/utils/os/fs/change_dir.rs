use crate::{async_command, print};
use anyhow::Result;

pub async fn change_dir(absolute_path: &str) -> Result<()> {
    let cmd = format!("cd {}", absolute_path);
    let msg = format!("Changed directory to {}", absolute_path);
    async_command(&cmd).await?;
    print("", &msg)?;
    Ok(())
}
