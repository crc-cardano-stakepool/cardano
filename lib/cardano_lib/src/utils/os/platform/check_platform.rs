use crate::async_command_pipe;
use anyhow::{anyhow, Result};

pub async fn check_platform() -> Result<String> {
    let platform = async_command_pipe("uname").await;
    match platform {
        Ok(platform) => Ok(platform),
        Err(e) => Err(anyhow!("{}", e)),
    }
}
