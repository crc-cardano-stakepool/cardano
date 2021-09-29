use crate::{async_command, check_user};
use anyhow::Result;

pub async fn async_user_command(command: &str) -> Result<()> {
    let user = check_user().await?;
    let cmd = format!("su - {} -c \"eval {}\"", user, command);
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::async_command;
    #[tokio::test]
    #[ignore]
    async fn test_async_command() {
        unimplemented!();
    }
}
