use crate::{source_shell, print, set_env, ENVS};
use anyhow::Result;

pub async fn export_shell_variables() -> Result<()> {
    print("", "Exporting shell variables")?;
    for (key, value) in ENVS.iter() {
        set_env(key, value);
    }
    source_shell().await?;
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
