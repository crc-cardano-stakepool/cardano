use anyhow::Result;

pub async fn compare_version(installed_version: &str, latest_version: &str) -> Result<bool> {
    if installed_version.eq(latest_version) {
        Ok(true)
    } else {
        Ok(false)
    }
}
