use crate::{compare_version, get_ghc_version};
use anyhow::Result;

pub async fn compare_ghc(installed_ghc: &str) -> Result<bool> {
    let version = get_ghc_version();
    compare_version(installed_ghc, version).await
}

#[cfg(test)]
mod test {
    // use crate::compare_ghc;
    #[tokio::test]
    #[ignore]
    async fn test_compare_ghc() {
        unimplemented!();
    }
}
