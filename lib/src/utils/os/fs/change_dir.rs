use crate::async_command;
use anyhow::Result;

pub async fn change_dir(absolute_path: &str) -> Result<()> {
    let cmd = format!("cd {}", absolute_path);
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::change_dir;
    #[tokio::test]
    #[ignore]
    async fn test_change_dir() {
        unimplemented!();
    }
}
