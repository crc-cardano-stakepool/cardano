use crate::async_command_pipe;
use anyhow::Result;

pub async fn get_bin_path(bin: &str) -> Result<String> {
    let cmd = format!("command -v {}", bin);
    async_command_pipe(&cmd).await
}
