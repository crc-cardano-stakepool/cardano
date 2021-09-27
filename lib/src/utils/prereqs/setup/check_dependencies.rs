use crate::{check_cabal, check_ghc, check_ghcup, print};
use anyhow::Result;

pub async fn check_dependencies() -> Result<()> {
    print("", "Checking dependencies")?;
    check_ghcup().await?;
    check_ghc().await?;
    check_cabal().await?;
    Ok(())
}
