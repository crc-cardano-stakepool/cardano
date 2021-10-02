use crate::{
    build_component, check_install, check_installed_version,
    check_latest_version, check_root, copy_binary, is_bin_installed,
    prepare_build, print, print_emoji, proceed, source_shell,
};
use anyhow::Result;
use console::Emoji;
use sudo::escalate_if_needed;

pub async fn install_component(component: &str) -> Result<()> {
    if let Ok(false) = check_root() {
        match escalate_if_needed() {
            Ok(user) => {
                let msg = format!("Running as {:#?}", user);
                print("", &msg)?
            }
            Err(_) => print("", "Failed obtaining root privileges")?,
        }
    } else if !is_bin_installed(component).await? {
        let msg =
            format!("Do you want to install the latest {} binary?", component);
        if proceed(&msg)? {
            let msg = format!("Installing latest {}", component);
            print_emoji("white", &msg, Emoji("🤟", ""))?;
            prepare_build().await?;
            build_component(component).await?;
            copy_binary(component).await?;
            check_install(component).await?;
            source_shell().await?;
        } else {
            let msg = format!("Aborted {} installation", component);
            print_emoji("red", &msg, Emoji("", ""))?;
        }
    } else {
        let installed_version = check_installed_version(component).await?;
        let latest_version = check_latest_version(component).await?;
        if installed_version.eq(&latest_version) {
            let msg = format!(
                "Already installed latest {} (v{})",
                component, latest_version
            );
            print_emoji("green", &msg, Emoji("🙌🎉", ""))?;
        } else {
            let msg = format!(
                "Currently {} (v{}) is installed, but the latest version is {}",
                component, installed_version, latest_version
            );
            print_emoji("yellow", &msg, Emoji("⚠️", ""))?;
            let msg = format!(
                "Do you want to install the latest {} binary?",
                component
            );
            if proceed(&msg)? {
                let msg = format!("Installing latest {}", component);
                print_emoji("white", &msg, Emoji("🤟", ""))?;
                prepare_build().await?;
                build_component(component).await?;
                copy_binary(component).await?;
                check_install(component).await?;
            } else {
                let msg = format!("Aborted {} installation", component);
                print_emoji("red", &msg, Emoji("", ""))?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::install_component;
    #[tokio::test]
    #[ignore]
    async fn test_install_component() {
        unimplemented!();
    }
}
