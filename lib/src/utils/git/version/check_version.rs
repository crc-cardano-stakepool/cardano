use crate::{check_installed_version, check_latest_version, compare_version, print_emoji};
use anyhow::Result;
use console::Emoji;

pub async fn check_version(component: &str) -> Result<bool> {
    let latest_version = check_installed_version(component).await?;
    let installed_version = check_latest_version(component).await?;
    if compare_version(&installed_version, &latest_version).await? {
        Ok(true)
    } else {
        let msg = format!("{} is not installed", component);
        print_emoji("red", &msg, Emoji("😔", ""))?;
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn test_check_node_version() {
        unimplemented!();
    }
    #[test]
    #[ignore]
    fn test_fetch_latest_node_version() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_fetch_installed_node_version() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_compare_latest_node_version() {
        unimplemented!();
    }
}
