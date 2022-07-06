use anyhow::Result;

use crate::{
    async_command, check_env, check_repo, setup_work_dir, CARDANO_URL,
};

pub async fn update_cli() -> Result<()> {
    log::info!("Updating the CLI");
    setup_work_dir()?;
    let cardano_path = check_env("CARDANO_DIR")?;
    check_repo(CARDANO_URL, &cardano_path).await?;
    let cmd = format!("cd {cardano_path} && git checkout main && git pull && cargo install --path src/bin");
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[ignore]
    pub async fn test_update_cli() -> Result<()> {
        update_cli().await?;
        Ok(())
    }
}
