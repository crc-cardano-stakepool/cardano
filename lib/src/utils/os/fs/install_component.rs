use crate::{
    build_latest_component, check_dir, check_root, check_version, check_work_dir, clone_cardano_repo, configure_build,
    copy_binary, prepare_build, print, print_emoji, proceed, setup_packages, setup_shell,
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
    } else if !check_version(component).await? {
        let msg = format!("Do you want to install the latest {} binary?", component);
        if proceed(&msg)? {
            let msg = format!("Installing latest {}", component);
            print_emoji("white", &msg, Emoji("🤟", ""))?;
            check_dir(&check_work_dir().await?).await?;
            setup_packages().await?;
            setup_shell().await?;
            prepare_build().await?;
            clone_cardano_repo(component).await?;
            configure_build(component).await?;
            build_latest_component(component).await?;
            copy_binary(component).await?;
        } else {
            let msg = format!("Aborted {} installation", component);
            print_emoji("red", &msg, Emoji("", ""))?;
        }
    } else {
        let msg = format!("The latest {} version is installed", component);
        print_emoji("green", &msg, Emoji("🙌🎉", ""))?;
    }
    Ok(())
}
