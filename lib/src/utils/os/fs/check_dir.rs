use crate::create_dir;
use anyhow::Result;
use std::path::Path;

pub async fn check_dir(absolute_path: &str) -> Result<()> {
    if !Path::new(absolute_path).is_dir() {
        create_dir(absolute_path).await?;
    }
    Ok(())
}
