use crate::{check_installed_ghc, compare_ghc, install_ghc, print};
use anyhow::Result;

pub async fn check_ghc() -> Result<()> {
    print("", "Checking GHC")?;
    let ghc = check_installed_ghc().await?;
    if compare_ghc(&ghc) {
        print("green", "GHC is installed")
    } else {
        let msg = format!(
            "Currently GHC v{} is installed, installing correct version of GHC",
            ghc
        );
        print("yellow", &msg)?;
        install_ghc().await
    }
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
