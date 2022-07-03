use crate::{async_user_command, check_env, get_cabal_version, get_ghc_version, GHCUP_URL};
use anyhow::{anyhow, Result};
use std::path::Path;

pub async fn check_ghcup() -> Result<()> {
    log::debug!("Checking GHCup");
    let ghcup_dir = check_env("GHCUP_DIR")?;
    let ghcup_bin = check_env("GHCUP_BIN")?;
    let ghcup_dir = Path::new(&ghcup_dir);
    let ghcup_bin = Path::new(&ghcup_bin);
    if ghcup_dir.is_dir() {
        if ghcup_bin.is_file() {
            return Ok(());
        }
        return Err(anyhow!("Failed installing GHCup"));
    }
    install_ghcup().await
}

pub async fn install_ghcup() -> Result<()> {
    log::info!("Installing GHCup");
    let ghc_version = get_ghc_version().await?;
    let cabal_version = get_cabal_version().await?;
    let non_interactive = "export BOOTSTRAP_HASKELL_NONINTERACTIVE=1";
    let ghc = format!("export BOOTSTRAP_HASKELL_GHC_VERSION={ghc_version}");
    let cabal = format!("export BOOTSTRAP_HASKELL_CABAL_VERSION={cabal_version}");
    let call = format!("$(curl --proto '=https' --tlsv1.2 -sSf {GHCUP_URL})");
    let cmd = format!("\n{non_interactive}\n{ghc}\n{cabal}\n{call}");
    async_user_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_install_ghcup() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_get_ghcup_install_url() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_ghcup() {
        unimplemented!();
    }
}
