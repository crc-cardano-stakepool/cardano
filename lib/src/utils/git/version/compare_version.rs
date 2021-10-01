pub fn compare_version(installed_version: &str, latest_version: &str) -> bool {
    installed_version.eq(latest_version)
}

#[cfg(test)]
mod test {
    // use crate::compare_version;
    #[tokio::test]
    #[ignore]
    async fn test_compare_version() {
        unimplemented!();
    }
}
