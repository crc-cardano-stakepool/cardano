use crate::{install_ghcup, print};
use anyhow::Result;

pub async fn check_ghcup() -> Result<()> {
    print("", "Checking GHCup")?;
    install_ghcup().await?;
    Ok(())
}
