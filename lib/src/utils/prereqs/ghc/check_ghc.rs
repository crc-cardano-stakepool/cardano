use crate::{install_ghc, print};
use anyhow::Result;

pub async fn check_ghc() -> Result<()> {
    print("", "Checking GHC")?;
    install_ghc().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::check_ghc;
    #[tokio::test]
    #[ignore]
    async fn test_check_ghc() {
        unimplemented!();
    }
}
