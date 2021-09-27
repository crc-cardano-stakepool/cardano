use crate::print;
use anyhow::Result;

pub async fn check_cabal() -> Result<()> {
    print("", "Checking Cabal")?;
    Ok(())
}
