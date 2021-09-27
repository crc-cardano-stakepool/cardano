use crate::print;
use anyhow::Result;

pub async fn install_ghcup() -> Result<()> {
    print("", "Installing GHCup")?;
    Ok(())
}
