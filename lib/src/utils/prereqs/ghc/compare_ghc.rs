use crate::get_ghc_version;

pub fn compare_ghc(installed_ghc: &str) -> bool {
    let version = get_ghc_version();
    installed_ghc.eq(version)
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
