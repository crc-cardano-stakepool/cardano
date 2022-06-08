use crate::{ask_shell_config, check_shell, match_shell, setup_env};
use anyhow::Result;

pub async fn setup_shell() -> Result<()> {
    let shell = check_shell().await?;
    match_shell(&shell)?;
    ask_shell_config().await?;
    setup_env().await
}

#[cfg(test)]
mod test {
    // use crate::setup_shell;
    #[tokio::test]
    #[ignore]
    async fn test_setup_shell() {
        unimplemented!();
    }
}
