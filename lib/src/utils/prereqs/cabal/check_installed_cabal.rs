use crate::{async_command_pipe, check_env, file_exists, install_cabal};
use anyhow::Result;
use async_recursion::async_recursion;

#[async_recursion]
pub async fn check_installed_cabal() -> Result<String> {
    let cabal = check_env("CABAL_BIN")?;
    if file_exists(&cabal) {
        let cmd = format!("{} -V | head -n1 | awk {}", cabal, "'{print $3}'");
        let installed_cabal = async_command_pipe(&cmd).await?;
        let installed_cabal = installed_cabal.trim().to_string();
        Ok(installed_cabal)
    } else {
        install_cabal().await?;
        check_installed_cabal().await
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
