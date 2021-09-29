use crate::{check_env, check_shell, match_shell, process_success};
use anyhow::Result;

pub async fn check_shell_config_env(pattern: &str) -> Result<bool> {
    let shell = check_shell().await?;
    match_shell(&shell)?;
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    let cmd = format!("grep -q {} {}", pattern, shell_profile_file);
    if process_success(&cmd).await? {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    // use crate::check_shell_config_env;
    #[tokio::test]
    #[ignore]
    async fn test_check_shell_config_env() {
        unimplemented!();
    }
}
