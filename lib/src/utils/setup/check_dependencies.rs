use crate::{check_cabal, check_ghc, check_ghcup, check_libsodium, print};
use anyhow::Result;

pub async fn check_dependencies() -> Result<()> {
    print("", "Checking dependencies")?;
    check_ghcup().await?;
    check_ghc().await?;
    check_cabal().await?;
    check_libsodium().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::check_dependencies;
    #[tokio::test]
    #[ignore]
    async fn test_check_dependencies() {
        unimplemented!();
    }
}
