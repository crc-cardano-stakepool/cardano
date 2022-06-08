use crate::{async_user_command, check_env, file_exists, get_cabal_version, get_ghc_version, is_dir, print, URLS};
use anyhow::{anyhow, Result};

pub async fn check_ghcup() -> Result<()> {
    print("", "Checking GHCup")?;
    let ghcup_dir = check_env("GHCUP_DIR")?;
    let ghcup_bin = check_env("GHCUP_BIN")?;
    if is_dir(&ghcup_dir) {
        if file_exists(&ghcup_bin) {
            print("green", "GHCup is installed")
        } else {
            Err(anyhow!("Failed installing GHCup"))
        }
    } else {
        print("red", "GHCup is not installed")?;
        install_ghcup().await
    }
}

pub fn get_ghcup_install_url() -> &'static str {
    if let Some(url) = URLS.get("ghcup") {
        url
    } else {
        "https://get-ghcup.haskell.org"
    }
}

pub async fn install_ghcup() -> Result<()> {
    print("", "Installing GHCup")?;
    let ghc_version = get_ghc_version().await?;
    let cabal_version = get_cabal_version().await?;
    let ghcup_install_url = get_ghcup_install_url();
    let non_interactive = "export BOOTSTRAP_HASKELL_NONINTERACTIVE=1";
    let ghc = format!("export BOOTSTRAP_HASKELL_GHC_VERSION={}", ghc_version);
    let cabal = format!("export BOOTSTRAP_HASKELL_CABAL_VERSION={}", cabal_version);
    let call = format!("$(curl --proto '=https' --tlsv1.2 -sSf {})", ghcup_install_url);
    let cmd = format!("\n{}\n{}\n{}\n{}", non_interactive, ghc, cabal, call);
    async_user_command(&cmd).await?;
    print("green", "Successfully installed GHCup")
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
