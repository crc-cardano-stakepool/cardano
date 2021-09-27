use crate::{install_ghc, print};
use anyhow::Result;

pub async fn check_ghc() -> Result<()> {
    print("", "Checking GHC")?;
    install_ghc().await?;
    Ok(())
}
