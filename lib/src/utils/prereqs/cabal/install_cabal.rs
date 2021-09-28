use crate::print;
use anyhow::Result;

pub async fn install_cabal() -> Result<()> {
    print("", "Installing Cabal")?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::install_cabal;
    #[tokio::test]
    #[ignore]
    async fn test_install_cabal() {
        unimplemented!();
    }
}
