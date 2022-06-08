use crate::{
    async_command, async_user_command, check_cabal, check_dir, check_env, check_ghc, check_ghcup,
    check_installed_version, check_latest_version, check_libsodium, check_root, check_user, check_work_dir, chownr,
    clone_component, copy_binary, file_exists, get_ghc_version, is_bin_installed, print, print_emoji, proceed,
    process_success_inherit, set_env, setup_packages, setup_shell, setup_work_dir, source_shell,
};
use anyhow::{anyhow, Result};
use console::Emoji;
use convert_case::{Case, Casing};
use sudo::escalate_if_needed;

pub async fn build_component(component: &str) -> Result<()> {
    clone_component(component).await?;
    let ghc_version = get_ghc_version().await?;
    let cabal = check_env("CABAL_BIN")?;
    let project_file = get_project_file(component).await?;
    let path = get_component_path(component).await?;
    update_cabal(&path, &cabal).await?;
    check_project_file(&project_file).await?;
    configure_build(component, &ghc_version, &path, &cabal).await?;
    update_project_file(component, &project_file).await?;
    build(component, &path, &cabal).await
}

async fn update_cabal(path: &str, cabal_path: &str) -> Result<()> {
    let cmd = format!("cd {} && {} update", path, cabal_path);
    print("", "Updating Cabal")?;
    async_user_command(&cmd).await?;
    print("green", "Updated Cabal successfully")
}

async fn check_project_file(project_file: &str) -> Result<()> {
    if file_exists(project_file) {
        let cmd = format!("rm {}", project_file);
        async_command(&cmd).await?;
        print("", "Removed project file")
    } else {
        Ok(())
    }
}

async fn build(component: &str, path: &str, cabal: &str) -> Result<()> {
    let user = check_user().await?;
    let cmd = format!("cd {} && {} build all", path, cabal);
    let cmd = format!("su - {} -c \"eval {}\"", user, cmd);
    let msg = format!("Building {}", component);
    print("", &msg)?;
    if process_success_inherit(&cmd).await? {
        let msg = format!("Successfully built {}", component);
        print("green", &msg)
    } else {
        Err(anyhow!("Failed building {}", component))
    }
}

pub async fn check_dependencies() -> Result<()> {
    print("", "Checking dependencies")?;
    check_ghcup().await?;
    check_ghc().await?;
    check_cabal().await?;
    check_libsodium().await
}

pub async fn check_install(component: &str) -> Result<()> {
    let msg = format!("Checking successful {} installation", component);
    print("", &msg)?;
    if let "cardano-node" = component {
        check_installed_version("cardano-cli").await?;
    }
    check_installed_version(component).await?;
    source_shell().await?;
    let msg = format!("Successfully installed {}", component);
    print_emoji("green", &msg, Emoji("🙌🎉", ""))
}

pub async fn configure_build(component: &str, ghc_version: &str, path: &str, cabal: &str) -> Result<()> {
    print("", "Configuring build")?;
    let ghc = check_env("GHC_BIN")?;
    let cmd = format!(
        "cd {} && {} configure --with-compiler={}-{}",
        path, cabal, ghc, ghc_version
    );
    async_user_command(&cmd).await?;
    let msg = format!("Configured build of {} successfully", component);
    print("green", &msg)
}

pub async fn get_component_path(component: &str) -> Result<String> {
    let env = format!("{}-dir", component);
    let converted = env.to_case(Case::UpperSnake);
    let path = check_env(&converted)?;
    Ok(path)
}

pub async fn get_project_file(component: &str) -> Result<String> {
    let path = get_component_path(component).await?;
    let project_file = format!("{}/cabal.project.local", path);
    Ok(project_file)
}

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

pub async fn prepare_build() -> Result<()> {
    print("", "Preparing build")?;
    check_dir(&check_work_dir().await?).await?;
    setup_packages().await?;
    setup_shell().await?;
    setup_work_dir().await?;
    check_dependencies().await
}

pub fn set_confirm(confirm: bool) {
    if confirm {
        set_env("CONFIRM", "true")
    } else {
        set_env("CONFIRM", "false")
    }
}

pub async fn update_project_file(component: &str, file_path: &str) -> Result<()> {
    let package = format!("echo \"package cardano-crypto-praos\" >> {}", file_path);
    let libsodium_flag = format!("echo \"  flags: -external-libsodium-vrf\" >> {}", file_path);
    async_command(&package).await?;
    async_command(&libsodium_flag).await?;
    let msg = format!("Updated project file of {}", component);
    chownr(file_path).await?;
    print("green", &msg)
}

#[cfg(test)]
mod test {
    // use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_update_project_file() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_prepare_build() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_install_component() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_project_file() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_component_path() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_configure_build() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_install() {
        unimplemented!();
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_check_dependencies() {
        unimplemented!();
    }
}
