use crate::check_user;
use anyhow::Result;
use tokio::process::Command;

pub async fn user_process_success(command: &str) -> Result<bool> {
    let user = check_user().await?;
    let cmd = format!("su - {} -c \"eval {}\"", user, command);
    let output = Command::new("sh").arg("-c").arg(&cmd).output().await?;
    Ok(output.status.success())
}

#[cfg(test)]
mod test {
    // use crate::process_success;
    #[tokio::test]
    #[ignore]
    async fn test_process_success() {
        unimplemented!();
    }
}
