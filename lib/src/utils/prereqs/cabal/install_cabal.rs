use crate::{async_command, check_env, get_cabal_version, print};
use anyhow::Result;

pub async fn install_cabal() -> Result<()> {
    print("", "Installing Cabal")?;
    let version = get_cabal_version();
    let msg = format!("Installing Cabal v{}", version);
    print("", &msg)?;
    let ghcup = check_env("GHCUP_BIN")?;
    let cmd = format!("{} install cabal {}", ghcup, version);
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::install_cabal;
    #[tokio::test]
    #[ignore]
    async fn test_install_cabal() {
        unimplemented!();
    }
}
