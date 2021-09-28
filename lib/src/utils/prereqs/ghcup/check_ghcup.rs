use crate::{install_ghcup, print};
use anyhow::Result;

pub async fn check_ghcup() -> Result<()> {
    print("", "Checking GHCup")?;
    install_ghcup().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::check_ghcup;
    #[tokio::test]
    #[ignore]
    async fn test_check_ghcup() {
        unimplemented!();
    }
}
