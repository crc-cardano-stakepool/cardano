use crate::{check_installed_version, print, print_emoji};
use anyhow::Result;
use console::Emoji;

pub async fn check_install(component: &str) -> Result<()> {
    let msg = format!("Checking successful {} installation", component);
    print("", &msg)?;
    if let "cardano-node" = component {
        check_installed_version("cardano-cli").await?;
    }
    check_installed_version(component).await?;
    let msg = format!("Successfully installed {}", component);
    print_emoji("green", &msg, Emoji("🙌🎉", ""))
}

#[cfg(test)]
mod test {
    // use crate::check_install;
    #[tokio::test]
    #[ignore]
    async fn test_check_install() {
        unimplemented!();
    }
}
