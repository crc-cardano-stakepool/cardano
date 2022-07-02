use crate::{
    async_command, check_cabal, check_env, check_ghc, check_ghcup, check_installed_version, check_latest_version, check_libsodium,
    check_secp256k1, clone_component, copy_binary, file_exists, get_ghc_version, is_bin_installed, proceed, process_success_inherit,
    set_env, setup_packages, setup_shell, setup_work_dir, source_shell, SystemRequirements,
};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};

pub async fn install_component(component: &str, confirm: bool) -> Result<()> {
    log::info!("Installing {component}");
    set_confirm(confirm);
    if !is_bin_installed(component).await? {
        check_confirm(component, confirm).await
    } else {
        install_if_not_up_to_date(component, confirm).await
    }
}

pub fn set_confirm(confirm: bool) {
    if confirm {
        set_env("CONFIRM", "true")
    } else {
        set_env("CONFIRM", "false")
    }
}

async fn check_confirm(component: &str, confirm: bool) -> Result<()> {
    if confirm {
        install(component).await
    } else {
        let latest = check_latest_version(component).await?;
        proceed_install(component, &latest).await
    }
}

async fn install_if_not_up_to_date(component: &str, confirm: bool) -> Result<()> {
    log::info!("Installing {component} or updating if there is a new version available");
    let installed = check_installed_version(component).await?;
    let latest = check_latest_version(component).await?;
    if installed.eq(&latest) {
        Ok(())
    } else {
        check_confirm(component, confirm).await
    }
}

async fn proceed_install(component: &str, latest: &str) -> Result<()> {
    if !SystemRequirements::check_requirements() {
        log::error!("System not officially supported, installation may fail")
    }
    let msg = format!("Do you want to install the latest {component} binary (v{latest})?");
    if proceed(&msg)? {
        install(component).await
    } else {
        Ok(())
    }
}

async fn install(component: &str) -> Result<()> {
    prepare_build().await?;
    build_component(component).await?;
    copy_binary(component).await?;
    check_install(component).await
}

pub async fn prepare_build() -> Result<()> {
    log::info!("Preparing build");
    setup_work_dir().await?;
    setup_packages().await?;
    setup_shell().await?;
    check_dependencies().await
}

pub async fn check_dependencies() -> Result<()> {
    log::info!("Checking build dependencies");
    check_ghcup().await?;
    check_ghc().await?;
    check_cabal().await?;
    check_libsodium().await?;
    check_secp256k1().await
}

pub async fn build_component(component: &str) -> Result<()> {
    log::info!("Building {component}");
    clone_component(component).await?;
    let ghc_version = get_ghc_version().await?;
    let cabal = check_env("CABAL_BIN")?;
    let project_file = get_project_file(component).await?;
    let path = get_component_path(component).await?;
    update_cabal(&path, &cabal).await?;
    check_project_file(&project_file).await?;
    configure_build(&ghc_version, &path, &cabal).await?;
    update_project_file(&project_file).await?;
    build(component, &path, &cabal).await
}

async fn update_cabal(path: &str, cabal_path: &str) -> Result<()> {
    log::info!("Updating Cabal");
    let cmd = format!("cd {path} && {cabal_path} update");
    async_command(&cmd).await?;
    Ok(())
}

async fn check_project_file(project_file: &str) -> Result<()> {
    log::info!("Checking if the project file already exists");
    if file_exists(project_file) {
        log::warn!("Project file already exists, removing it");
        let cmd = format!("rm {project_file}");
        async_command(&cmd).await?;
        Ok(())
    } else {
        log::debug!("Project file does not exist");
        Ok(())
    }
}

pub async fn configure_build(ghc_version: &str, path: &str, cabal: &str) -> Result<()> {
    log::info!("Configuring build");
    let ghc = check_env("GHC_BIN")?;
    let cmd = format!("cd {path} && {cabal} configure --with-compiler={ghc}-{ghc_version}");
    async_command(&cmd).await?;
    Ok(())
}

pub async fn update_project_file(file_path: &str) -> Result<()> {
    log::info!("Updating the project file at {file_path}");
    let package = format!("echo \"package cardano-crypto-praos\" >> {file_path}");
    let libsodium_flag = format!("echo \"  flags: -external-libsodium-vrf\" >> {file_path}");
    async_command(&package).await?;
    async_command(&libsodium_flag).await?;
    Ok(())
}

pub async fn get_component_path(component: &str) -> Result<String> {
    log::info!("Checking where the source reposity of {component} is");
    let env = format!("{component}-dir");
    let converted = env.to_case(Case::UpperSnake);
    let path = check_env(&converted)?;
    Ok(path)
}

pub async fn get_project_file(component: &str) -> Result<String> {
    log::info!("Getting the project file of the {component} source reposity");
    let path = get_component_path(component).await?;
    let project_file = format!("{path}/cabal.project.local");
    Ok(project_file)
}

async fn build(component: &str, path: &str, cabal: &str) -> Result<()> {
    log::info!("Building {component}");
    let cmd = format!("cd {path} && {cabal} build all");
    if process_success_inherit(&cmd).await? {
        log::debug!("Successfully built {component}");
        Ok(())
    } else {
        Err(anyhow!("Failed building {component}"))
    }
}

pub async fn check_install(component: &str) -> Result<()> {
    if let "cardano-node" = component {
        check_installed_version("cardano-cli").await?;
    }
    check_installed_version(component).await?;
    source_shell().await?;
    Ok(())
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
