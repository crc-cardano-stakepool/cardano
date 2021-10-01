use crate::{
    check_installed_version, check_latest_version, compare_version, print_emoji,
};
use anyhow::Result;
use console::Emoji;

pub async fn check_version(component: &str) -> Result<bool> {
    let latest_version = check_installed_version(component).await?;
    let installed_version = check_latest_version(component).await;
    if let Ok(version) = installed_version {
        if compare_version(&version, &latest_version) {
            Ok(true)
        } else {
            let msg = format!(
                "Currently {} (v{}) is installed, but the latest version is {}",
                component, version, latest_version
            );
            print_emoji("yellow", &msg, Emoji("⚠️", ""))?;
            Ok(false)
        }
    } else {
        let msg = format!("{} is not installed", component);
        print_emoji("red", &msg, Emoji("😔", ""))?;
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    // use crate::check_version;
    #[tokio::test]
    #[ignore]
    async fn test_check_version() {
        unimplemented!();
    }
}
