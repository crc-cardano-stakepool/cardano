use anyhow::Result;

pub async fn compare_version(installed_version: &str, latest_version: &str) -> Result<bool> {
    if installed_version.eq(latest_version) {
        Ok(true)
    } else {
        Ok(false)
    }
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
