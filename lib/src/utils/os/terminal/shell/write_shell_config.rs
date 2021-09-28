use crate::{async_command, check_env, print};
use anyhow::Result;

pub async fn write_shell_config(value: &str) -> Result<()> {
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    let append_string = format!("$(cat << 'EOF'\n{}\nEOF\n)", value);
    let cmd = format!("echo \"{}\" >> {}", append_string, shell_profile_file);
    let msg = format!("Added line to {}: {}", shell_profile_file, value);
    print("green", &msg)?;
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::write_shell_config;
    #[tokio::test]
    #[ignore]
    async fn test_write_shell_config() {
        unimplemented!();
    }
}
