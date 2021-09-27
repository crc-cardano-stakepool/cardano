use crate::print;
use anyhow::Result;

pub async fn install_ghc() -> Result<()> {
    print("", "Installing GHC")?;
    Ok(())
}
