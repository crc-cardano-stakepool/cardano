use crate::{change_dir, chownr, print};
use anyhow::Result;
use tokio::fs::create_dir_all;

pub async fn create_dir(dir_name: &str, absolute_path: &str) -> Result<()> {
    let msg = format!("Creating {} in {}", dir_name, absolute_path);
    print("", &msg)?;
    create_dir_all(absolute_path).await?;
    chownr(absolute_path).await?;
    change_dir(absolute_path).await?;
    Ok(())
}
