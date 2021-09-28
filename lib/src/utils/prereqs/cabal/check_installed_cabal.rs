use crate::{async_command_pipe, check_env, file_exists};
use anyhow::{anyhow, Result};

pub async fn check_installed_cabal() -> Result<String> {
    let cabal = check_env("CABAL_BIN")?;
    if file_exists(&cabal) {
        let cmd = format!("{} -V | head -n1 | awk {}", cabal, "'{print $3}'");
        let installed_cabal = async_command_pipe(&cmd).await?;
        let installed_cabal = installed_cabal.trim().to_string();
        Ok(installed_cabal)
    } else {
        Err(anyhow!("Cabal is not installed"))
    }
}

#[cfg(test)]
mod test {
    // use crate::check_installed_cabal;
    #[tokio::test]
    #[ignore]
    async fn test_check_installed_cabal() {
        unimplemented!();
    }
}
