use crate::{async_command, print, PATHS};
use anyhow::Result;

pub async fn export_shell_variables() -> Result<()> {
    print("", "Exporting shell variables")?;
    for (_, value) in PATHS.iter() {
        async_command(value).await?;
    }
    Ok(())
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
