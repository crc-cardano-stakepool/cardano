use crate::{async_command_pipe, check_env, file_exists};
use anyhow::{anyhow, Result};

pub async fn check_installed_ghc() -> Result<String> {
    let ghc = check_env("GHC_BIN")?;
    if file_exists(&ghc) {
        let cmd = format!("{} -V | awk {}", ghc, "'{print $8}'");
        let installed_ghc = async_command_pipe(&cmd).await?;
        let installed_ghc = installed_ghc.trim().to_string();
        Ok(installed_ghc)
    } else {
        Err(anyhow!("GHC is not installed"))
    }
}

#[cfg(test)]
mod test {
    // use crate::check_installed_ghc;
    #[tokio::test]
    #[ignore]
    async fn test_check_installed_ghc() {
        unimplemented!();
    }
}
