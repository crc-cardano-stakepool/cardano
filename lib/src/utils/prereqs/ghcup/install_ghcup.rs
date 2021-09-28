use crate::print;
use anyhow::Result;

pub async fn install_ghcup() -> Result<()> {
    print("", "Installing GHCup")?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::install_ghcup;
    #[tokio::test]
    #[ignore]
    async fn test_install_ghcup() {
        unimplemented!();
    }
}
