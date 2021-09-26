use crate::{check_env, process_success};
use anyhow::Result;

pub async fn check_shell_config_env(pattern: &str) -> Result<bool> {
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    let cmd = format!("grep -q {} {}", pattern, shell_profile_file);
    if process_success(&cmd).await? {
        Ok(true)
    } else {
        Ok(false)
    }
}
