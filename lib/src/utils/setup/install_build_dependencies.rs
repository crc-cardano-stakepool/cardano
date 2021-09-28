use crate::{check_cabal, check_ghc, check_ghcup, install_libsodium, print, setup_work_dir};
use anyhow::Result;

pub async fn install_build_dependencies() -> Result<()> {
    print("", "Checking dependencies")?;
    setup_work_dir().await?;
    check_ghcup().await?;
    check_ghc().await?;
    check_cabal().await?;
    install_libsodium().await?;
    print("green", "Successfully installed dependencies")?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::install_build_dependencies;
    #[tokio::test]
    #[ignore]
    async fn test_install_build_dependencies() {
        unimplemented!();
    }
}
