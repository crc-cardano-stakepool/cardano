use crate::print;
use anyhow::Result;

pub async fn install_ghc() -> Result<()> {
    print("", "Installing GHC")?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::install_ghc;
    #[tokio::test]
    #[ignore]
    async fn test_install_ghc() {
        unimplemented!();
    }
}
