use crate::check_user;
use anyhow::Result;

pub async fn get_bin_path(bin: &str) -> Result<String> {
    let user = check_user().await?;
    let path = format!("/home/{}/.local/bin/{}", user, bin);
    Ok(path)
}
