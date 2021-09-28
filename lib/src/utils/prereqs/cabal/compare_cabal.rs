use crate::get_cabal_version;
use anyhow::Result;

pub async fn compare_cabal(installed_cabal: &str) -> Result<bool> {
    let version = get_cabal_version();
    if version == installed_cabal {
        Ok(true)
    } else {
        Ok(false)
    }
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
