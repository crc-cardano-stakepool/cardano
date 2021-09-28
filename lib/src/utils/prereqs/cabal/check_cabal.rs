use crate::{check_env, compare_cabal, file_exists, install_cabal, print};
use anyhow::Result;

pub async fn check_cabal() -> Result<()> {
    print("", "Checking Cabal")?;
    let cabal = check_env("CABAL_BIN")?;
    if file_exists(&cabal) {
        compare_cabal(&cabal).await?;
    } else {
        install_cabal().await?;
    }
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
