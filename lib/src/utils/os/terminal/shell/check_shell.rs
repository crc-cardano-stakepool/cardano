use crate::check_env;
use anyhow::Result;

pub async fn check_shell() -> Result<String> {
    let shell = check_env("SHELL")?;
    Ok(shell)
}
