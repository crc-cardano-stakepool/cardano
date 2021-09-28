use crate::print;
use anyhow::Result;

pub async fn check_cabal() -> Result<()> {
    print("", "Checking Cabal")?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::check_cabal;
    #[tokio::test]
    #[ignore]
    async fn test_check_cabal() {
        unimplemented!();
    }
}
