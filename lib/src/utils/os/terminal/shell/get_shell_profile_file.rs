use crate::{check_env, check_shell, match_shell};
use anyhow::Result;

pub async fn get_shell_profile_file() -> Result<String> {
    match_shell(&check_shell().await?)?;
    Ok(check_env("SHELL_PROFILE_FILE")?)
}
