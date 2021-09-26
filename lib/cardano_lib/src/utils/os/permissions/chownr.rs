use crate::{async_command, check_user};
use anyhow::Result;

pub async fn chownr(absolute_path: &str) -> Result<()> {
    let user = check_user().await?;
    let user = user.trim();
    let cmd = format!("chown -R {}:{} {}", user, user, absolute_path);
    async_command(&cmd).await?;
    Ok(())
}
