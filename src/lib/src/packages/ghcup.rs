use crate::{
    async_command, check_env, check_user, drop_privileges, Cabal, Ghc,
    GHCUP_URL,
};
use anyhow::{anyhow, Result};
use std::path::Path;

#[derive(Debug)]
pub struct Ghcup;
impl Ghcup {
    pub fn check_ghcup() -> Result<()> {
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
        Self::install_ghcup()
    }
    pub fn install_ghcup() -> Result<()> {
        log::info!("Installing GHCup");
        let user = check_user()?;
        let ghc_version = Ghc::get_ghc_version()?;
        let cabal_version = Cabal::get_cabal_version()?;
        let non_interactive = "export BOOTSTRAP_HASKELL_NONINTERACTIVE=1";
        let ghc = format!("export BOOTSTRAP_HASKELL_GHC_VERSION={ghc_version}");
        let cabal =
            format!("export BOOTSTRAP_HASKELL_CABAL_VERSION={cabal_version}");
        let call =
            format!("$(curl --proto '=https' --tlsv1.2 -sSf {GHCUP_URL})");
        let cmd = format!("\n{non_interactive}\n{ghc}\n{cabal}\n{call}");
        let cmd = format!("sudo su - {user} -c \"eval {cmd}\"");
        async_command(&cmd)?;
        drop_privileges()?;
        Ok(())
    }
}
#[cfg(test)]
mod test {
    // use super::*;
}
