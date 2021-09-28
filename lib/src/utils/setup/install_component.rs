use crate::{
    build_component, check_install, check_root, check_version, clone_cardano_repo, copy_binary, prepare_build, print,
    print_emoji, proceed,
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
            prepare_build().await?;
            clone_cardano_repo(component).await?;
            build_component(component).await?;
            copy_binary(component).await?;
            check_install(component).await?;
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

#[cfg(test)]
mod test {
    // use crate::install_component;
    #[tokio::test]
    #[ignore]
    async fn test_install_component() {
        unimplemented!();
    }
}
