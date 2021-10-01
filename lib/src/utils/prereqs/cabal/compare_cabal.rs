use crate::{compare_version, get_cabal_version};

pub fn compare_cabal(installed_cabal: &str) -> bool {
    let version = get_cabal_version();
    compare_version(installed_cabal, version)
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
