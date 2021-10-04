use crate::{check_user, file_exists};
use anyhow::Result;

pub async fn is_bin_installed(bin: &str) -> Result<bool> {
    let user = check_user().await?;
    let file = format!("/home/{}/.local/bin/{}", user, bin);
    Ok(file_exists(&file))
}
