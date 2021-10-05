use crate::{change_shell_config, check_env, export_shell_variables, print, proceed};
use anyhow::{anyhow, Result};

pub async fn ask_shell_config() -> Result<()> {
    let shell = check_env("MY_SHELL")?;
    let shell_file = check_env("SHELL_PROFILE_FILE")?;
    if shell.is_empty() || shell_file.is_empty() {
        return Err(anyhow!("No shell found"));
    }
    let msg = format!("Detected {}", shell);
    print("green", &msg)?;
    check_ask_shell_confirm(&shell, &shell_file).await
}

async fn check_ask_shell_confirm(shell: &str, shell_file: &str) -> Result<()> {
    let confirm = check_env("CONFIRM")?;
    let msg = format!(
        "Do you want to automatically add the required PATH variables to {}",
        shell_file
    );
    if confirm == "false" && proceed(&msg)? {
        let msg = format!("Proceeding to add path variables for {} to {}", shell, shell_file);
        print("magenta", &msg)?;
        change_shell_config().await
    } else {
        print("yellow", "Skipped adding path variables")?;
        export_shell_variables().await
    }
}

#[cfg(test)]
mod test {
    // use crate::ask_shell_config;
    #[tokio::test]
    #[ignore]
    async fn test_ask_shell_config() {
        unimplemented!();
    }
}
