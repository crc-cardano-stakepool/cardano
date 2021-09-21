use super::process::async_command_pipe;
use anyhow::Result;

pub async fn update() -> Result<()> {
    println!("Updating");
    async_command_pipe("apt update -y && apt upgrade -y").await?;
    println!("Finished updating");
    Ok(())
}
