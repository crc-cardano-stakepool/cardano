use crate::{get_shell_profile_file, process_success};
use anyhow::Result;

pub async fn check_shell_config_env(pattern: &str) -> Result<bool> {
    let shell_profile_file = get_shell_profile_file().await?;
    let cmd = format!("grep -q {} {}", pattern, shell_profile_file);
    process_success(&cmd).await
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
