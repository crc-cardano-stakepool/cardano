use crate::process_success;
use anyhow::Result;

pub async fn is_bin_installed(bin: &str) -> Result<bool> {
    let cmd = format!("command -v {}", bin);
    process_success(&cmd).await
}
