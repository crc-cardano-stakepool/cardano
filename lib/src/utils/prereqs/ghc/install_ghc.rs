use crate::{async_user_command, check_env, get_ghc_version, print};
use anyhow::Result;

pub async fn install_ghc() -> Result<()> {
    let version = get_ghc_version().await?;
    let msg = format!("Installing GHC v{}", version);
    print("", &msg)?;
    let ghcup = check_env("GHCUP_BIN")?;
    let cmd = format!("{} install ghc {}", ghcup, version);
    async_user_command(&cmd).await?;
    let cmd = format!("{} set ghc {}", ghcup, version);
    async_user_command(&cmd).await?;
    print("green", "Successfully installed GHC")
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
