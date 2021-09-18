use super::process::async_command;
use anyhow::Result;

pub async fn update() -> Result<()> {
    println!("Updating");
    async_command("apt update -y && apt upgrade -y").await?;
    Ok(())
}
