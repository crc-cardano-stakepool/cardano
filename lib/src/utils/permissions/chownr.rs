use crate::{async_command, check_user};
use anyhow::{anyhow, Result};

// TODO: Use standard library instead
pub async fn chownr(absolute_path: &str) -> Result<()> {
    let user = check_user().await?;
    let user = user.trim();
    let cmd = format!("chown -R {}:{} {}", user, user, absolute_path);
    if async_command(&cmd).await.is_ok() {
        Ok(())
    } else {
        Err(anyhow!("Failed adjusting permissions of {}", absolute_path))
    }
}

#[cfg(test)]
mod test {
    // use crate::chownr;
    #[tokio::test]
    #[ignore]
    async fn test_chownr() {
        unimplemented!();
    }
}
