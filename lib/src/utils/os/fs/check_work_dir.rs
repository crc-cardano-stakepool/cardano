use crate::{check_home_dir, set_env};
use anyhow::Result;

pub async fn check_work_dir() -> Result<String> {
    let home = check_home_dir().await?;
    let install_directory = format!("{}/.cardano", home);
    set_env("WORK_DIR", &install_directory);
    Ok(install_directory)
}
