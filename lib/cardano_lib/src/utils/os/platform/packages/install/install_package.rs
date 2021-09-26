use crate::os::process::async_command;
use crate::os::terminal::stdout::print;
use anyhow::{anyhow, Result};

pub async fn install_package(package_manager: &str, package: &str) -> Result<()> {
    let msg = format!("{} is not installed", package);
    print("red", &msg)?;
    let cmd = format!("sudo {} install {} -y", package_manager, package);
    let process = async_command(&cmd).await;
    match process {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed installing {} with error: {}", package, e)),
    }
}
