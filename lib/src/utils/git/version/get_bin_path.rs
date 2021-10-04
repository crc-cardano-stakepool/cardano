use crate::{check_env};
use anyhow::Result;

pub async fn get_bin_path(bin: &str) -> Result<String> {
    let install_dir = check_env("INSTALL_DIR")?;
    let path = format!("{}/{}", install_dir, bin);
    Ok(path)
}
