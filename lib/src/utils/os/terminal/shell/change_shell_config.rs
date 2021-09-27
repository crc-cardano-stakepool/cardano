use crate::{check_shell_config_env, print, write_shell_config, PATHS};
use anyhow::Result;

pub async fn change_shell_config() -> Result<()> {
    print("", "Checking for shell configuration")?;
    for (key, value) in PATHS.iter() {
        if let Ok(false) = check_shell_config_env(key).await {
            write_shell_config(value).await?;
        }
    }
    print("green", "Shell configured")?;
    Ok(())
}
