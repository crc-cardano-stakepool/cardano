use crate::{async_command_pipe, check_env, file_exists, install_ghc};
use anyhow::Result;
use async_recursion::async_recursion;

#[async_recursion]
pub async fn check_installed_ghc() -> Result<String> {
    let ghc = check_env("GHC_BIN")?;
    if file_exists(&ghc) {
        let cmd = format!("{} -V | awk {}", ghc, "'{print $8}'");
        let installed_ghc = async_command_pipe(&cmd).await?;
        let installed_ghc = installed_ghc.trim().to_string();
        Ok(installed_ghc)
    } else {
        install_ghc().await?;
        check_installed_ghc().await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::setup_env;
    #[tokio::test]
    async fn test_check_installed_ghc() -> Result<()> {
        setup_env().await?;
        let version = check_installed_ghc().await?;
        println!("{version}");
        assert_eq!(version, "8.10.7");
        Ok(())
    }
}
