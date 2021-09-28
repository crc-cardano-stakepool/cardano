use crate::{async_command, check_env, get_ghc_version, print};
use anyhow::Result;

pub async fn install_ghc() -> Result<()> {
    let version = get_ghc_version();
    let msg = format!("Installing GHC v{}", version);
    print("", &msg)?;
    let ghcup = check_env("GHCUP_BIN")?;
    let cmd = format!("{} install ghc {}", ghcup, version);
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::install_ghc;
    #[tokio::test]
    #[ignore]
    async fn test_install_ghc() {
        unimplemented!();
    }
}
