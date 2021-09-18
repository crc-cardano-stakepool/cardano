use super::process::async_command;
use anyhow::Result;

pub async fn update_os_packages() -> Result<()> {
    println!("Updating");
    async_command("apt update -y && apt upgrade -y").await?;
    Ok(())
}
