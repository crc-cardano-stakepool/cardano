use crate::{ask_shell_config, check_shell, match_shell};
use anyhow::Result;

pub async fn setup_shell() -> Result<()> {
    let shell = check_shell().await?;
    match_shell(&shell)?;
    ask_shell_config().await?;
    Ok(())
}
