use crate::get_cabal_version;

pub fn compare_cabal(installed_cabal: &str) -> bool {
    let version = get_cabal_version();
    installed_cabal.eq(version)
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
