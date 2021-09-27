use crate::{chownr, print};
use anyhow::Result;
use tokio::fs::create_dir_all;

pub async fn create_dir(dir_name: &str, absolute_path: &str) -> Result<()> {
    let msg = format!("Created {} directory in {}", dir_name, absolute_path);
    create_dir_all(absolute_path).await?;
    chownr(absolute_path).await?;
    print("", &msg)?;
    Ok(())
}
