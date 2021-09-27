use crate::{check_env, print};
use anyhow::Result;

pub async fn copy_binary(component: &str) -> Result<()> {
    let install_dir = check_env("INSTALL_DIR")?;
    let msg = format!("Copying {} binary to {}", component, install_dir);
    print("", &msg)?;
    Ok(())
}
