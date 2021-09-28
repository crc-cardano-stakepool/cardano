use crate::{check_env, compare_ghc, file_exists, install_ghc, print};
use anyhow::Result;

pub async fn check_ghc() -> Result<()> {
    print("", "Checking GHC")?;
    let ghc = check_env("GHC_BIN")?;
    if file_exists(&ghc) {
        compare_ghc(&ghc).await?;
    } else {
        install_ghc().await?;
    }
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
