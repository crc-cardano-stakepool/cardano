use crate::{check_env, check_shell, match_shell};
use anyhow::Result;

pub async fn get_shell_profile_file() -> Result<String> {
    let shell = check_shell().await?;
    match_shell(&shell)?;
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    Ok(shell_profile_file)
}
