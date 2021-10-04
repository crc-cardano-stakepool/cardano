use crate::{async_command, check_shell_config_env, print, write_shell_config, PATHS};
use anyhow::Result;

pub async fn change_shell_config() -> Result<()> {
    print("", "Checking for shell configuration")?;
    for (key, value) in PATHS.iter() {
        if let Ok(false) = check_shell_config_env(key).await {
            write_shell_config(value).await?;
            async_command(value).await?;
        }
    }
    print("green", "Shell configured")
}

#[cfg(test)]
mod test {
    // use crate::change_shell_config;
    #[tokio::test]
    #[ignore]
    async fn test_change_shell_config() {
        unimplemented!();
    }
}
