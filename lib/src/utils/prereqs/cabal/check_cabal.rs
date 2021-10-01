use crate::{check_installed_cabal, compare_cabal, install_cabal, print};
use anyhow::Result;

pub async fn check_cabal() -> Result<()> {
    print("", "Checking Cabal")?;
    let cabal = check_installed_cabal().await?;
    if compare_cabal(&cabal) {
        print("green", "Cabal is installed")
    } else {
        let msg = format!(
            "Currently Cabal v{} is installed, installing correct version of Cabal",
            cabal
        );
        print("yellow", &msg)?;
        install_cabal().await
    }
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
