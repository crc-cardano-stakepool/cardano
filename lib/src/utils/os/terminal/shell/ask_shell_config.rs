use crate::{
    change_shell_config, check_env, export_shell_variables, print, proceed,
};
use anyhow::Result;

pub async fn ask_shell_config() -> Result<()> {
    let shell = check_env("MY_SHELL")?;
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    if shell.is_empty() || shell_profile_file.is_empty() {
        panic!("No shell found")
    }
    let msg = format!("Detected {}", shell);
    print("green", &msg)?;
    let msg = format!(
        "Do you want to automatically add the required PATH variables to {}",
        shell_profile_file
    );
    if proceed(&msg)? {
        let msg = format!(
            "Proceeding to add path variables for {} to {}",
            shell, shell_profile_file
        );
        print("magenta", &msg)?;
        change_shell_config().await?;
    } else {
        print(
            "red",
            "Skipped adding path variables, setting at runtime manually",
        )?;
        export_shell_variables().await?;
    }
    Ok(())
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
