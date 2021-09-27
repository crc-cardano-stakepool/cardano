use crate::print;
use anyhow::Result;

pub async fn install_cabal() -> Result<()> {
    print("", "Installing Cabal")?;
    Ok(())
}
