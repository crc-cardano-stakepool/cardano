use crate::chownr;
use anyhow::Result;
use tokio::fs::create_dir_all;

pub async fn create_dir(absolute_path: &str) -> Result<()> {
    create_dir_all(absolute_path).await?;
    chownr(absolute_path).await?;
    Ok(())
}
