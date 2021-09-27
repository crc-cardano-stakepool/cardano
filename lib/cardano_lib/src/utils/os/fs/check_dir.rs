use crate::{create_dir, print};
use anyhow::Result;
use std::path::Path;

pub async fn check_dir(dir_name: &str, absolute_path: &str) -> Result<()> {
    let msg = format!("Checking for {} in {}", dir_name, absolute_path);
    print("", &msg)?;
    if Path::new(absolute_path).is_dir() {
        let msg = format!("Found {} directory, skipped creating", dir_name);
        print("green", &msg)?;
    } else {
        create_dir(dir_name, absolute_path).await?;
    }
    Ok(())
}
