use crate::{compare_version, get_cabal_version};
use anyhow::Result;

pub async fn compare_cabal(installed_cabal: &str) -> Result<bool> {
    let version = get_cabal_version();
    compare_version(installed_cabal, version).await
}

#[cfg(test)]
mod test {
    // use crate::compare_cabal;
    #[tokio::test]
    #[ignore]
    async fn test_compare_cabal() {
        unimplemented!();
    }
}
