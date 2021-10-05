use crate::{
    build_component, check_install, check_installed_version, check_latest_version, check_root, copy_binary,
    is_bin_installed, prepare_build, print, print_emoji, proceed, set_confirm,
};
use anyhow::Result;
use console::Emoji;
use sudo::escalate_if_needed;

pub async fn install_component(component: &str, confirm: bool) -> Result<()> {
    set_confirm(confirm);
    if !check_root()? {
        match escalate_if_needed() {
            Ok(user) => {
                let msg = format!("Running as {:#?}", user);
                print("", &msg)
            }
            Err(_) => print("", "Failed obtaining root privileges"),
        }
    } else if !is_bin_installed(component).await? {
        check_confirm(component, confirm).await
    } else {
        install_if_not_up_to_date(component, confirm).await
    }
}

async fn install_if_not_up_to_date(component: &str, confirm: bool) -> Result<()> {
    let installed = check_installed_version(component).await?;
    let latest = check_latest_version(component).await?;
    if installed.eq(&latest) {
        let msg = format!("Already installed latest {} (v{})", component, latest);
        print_emoji("green", &msg, Emoji("🙌🎉", ""))
    } else {
        let msg = format!(
            "Currently {} (v{}) is installed, but the latest version is {}",
            component, installed, latest
        );
        print_emoji("yellow", &msg, Emoji("⚠️", ""))?;
        check_confirm(component, confirm).await
    }
}

async fn check_confirm(component: &str, confirm: bool) -> Result<()> {
    if confirm {
        install(component).await
    } else {
        proceed_install(component).await
    }
}

async fn install(component: &str) -> Result<()> {
    let msg = format!("Installing latest {}", component);
    print_emoji("white", &msg, Emoji("🤟", ""))?;
    prepare_build().await?;
    build_component(component).await?;
    copy_binary(component).await?;
    check_install(component).await
}

async fn proceed_install(component: &str) -> Result<()> {
    let msg = format!("Do you want to install the latest {} binary?", component);
    if proceed(&msg)? {
        install(component).await
    } else {
        let msg = format!("Aborted {} installation", component);
        print_emoji("red", &msg, Emoji("", ""))
    }
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
