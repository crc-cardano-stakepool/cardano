use std::path::{Path, PathBuf};

use crate::{
    absolute_ref_path_to_string, async_command, check_cabal, check_env, check_ghc, check_ghcup, check_installed_version,
    check_latest_version, check_libsodium, check_secp256k1, clone_component, copy_binary, get_ghc_version, is_bin_installed, proceed,
    process_success_inherit, set_env, setup_packages, setup_shell, setup_work_dir, source_shell, SystemRequirements,
};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};

pub async fn install_component(component: &str, confirm: bool) -> Result<()> {
    set_confirm(confirm);
    if !is_bin_installed(component).await? {
        return check_confirm(component, confirm).await;
    }
    install_if_not_up_to_date(component, confirm).await
}

pub fn set_confirm(confirm: bool) {
    if confirm {
        return set_env("CONFIRM", "true");
    }
    set_env("CONFIRM", "false")
}

async fn check_confirm(component: &str, confirm: bool) -> Result<()> {
    if confirm {
        return install(component).await;
    }
    let latest = check_latest_version(component).await?;
    proceed_install(component, &latest).await
}

async fn install_if_not_up_to_date(component: &str, confirm: bool) -> Result<()> {
    let installed = check_installed_version(component).await?;
    let latest = check_latest_version(component).await?;
    if !installed.eq(&latest) {
        return check_confirm(component, confirm).await;
    }
    log::info!("Latest {component} v{installed} is already installed");
    Ok(())
}

async fn proceed_install(component: &str, latest: &str) -> Result<()> {
    if !SystemRequirements::check_requirements() {
        log::error!("System not officially supported, installation may fail")
    }
    let msg = format!("Do you want to install the latest {component} binary (v{latest})?");
    if proceed(&msg)? {
        return install(component).await;
    }
    Ok(())
}

async fn install(component: &str) -> Result<()> {
    log::info!("Installing {component}");
    prepare_build().await?;
    build_component(component).await?;
    copy_binary(component).await?;
    check_install(component).await
}

pub async fn prepare_build() -> Result<()> {
    log::info!("Preparing build");
    setup_work_dir()?;
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
    let cabal = PathBuf::from(&cabal);
    let project_file = get_project_file(component)?;
    let path = get_component_path(component)?;
    update_cabal(&path, &cabal).await?;
    check_project_file(&project_file).await?;
    configure_build(&ghc_version, &path, &cabal).await?;
    update_project_file(&project_file).await?;
    build(component, &path, &cabal).await
}

async fn update_cabal<P: AsRef<Path>>(path: P, cabal_path: P) -> Result<()> {
    log::info!("Updating Cabal");
    let path = absolute_ref_path_to_string(&path)?;
    let cabal_path = absolute_ref_path_to_string(&cabal_path)?;
    let cmd = format!("cd {path} && {cabal_path} update");
    async_command(&cmd).await?;
    Ok(())
}

async fn check_project_file<P: AsRef<Path>>(project_file: P) -> Result<()> {
    log::debug!("Checking if the project file already exists");
    let path = absolute_ref_path_to_string(&project_file)?;
    if project_file.as_ref().is_file() {
        log::warn!("Project file already exists, removing it");
        let cmd = format!("rm {path}");
        async_command(&cmd).await?;
        return Ok(());
    }
    log::debug!("Project file does not exist");
    Ok(())
}

pub async fn configure_build<P: AsRef<Path>>(ghc_version: &str, path: P, cabal: P) -> Result<()> {
    log::info!("Configuring build");
    let ghc = check_env("GHC_BIN")?;
    let path = absolute_ref_path_to_string(&path)?;
    let cabal = absolute_ref_path_to_string(&cabal)?;
    let cmd = format!("cd {path} && {cabal} configure --with-compiler={ghc}-{ghc_version}");
    async_command(&cmd).await?;
    Ok(())
}

pub async fn update_project_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let file_path = absolute_ref_path_to_string(&path)?;
    if !path.as_ref().is_file() {
        return Err(anyhow!("The path {file_path} is not a file"));
    }
    log::info!("Updating the project file at {file_path}");
    let package = format!("echo \"package cardano-crypto-praos\" >> {file_path}");
    let libsodium_flag = format!("echo \"  flags: -external-libsodium-vrf\" >> {file_path}");
    async_command(&package).await?;
    async_command(&libsodium_flag).await?;
    Ok(())
}

pub fn get_component_path(component: &str) -> Result<PathBuf> {
    log::debug!("Checking where the source reposity of {component} is");
    let env = format!("{component}_dir");
    let converted = env.to_case(Case::UpperSnake);
    let path = check_env(&converted)?;
    let path = PathBuf::from(&path);
    Ok(path)
}

pub fn get_project_file(component: &str) -> Result<PathBuf> {
    log::debug!("Getting the project file of the {component} source reposity");
    let mut path = get_component_path(component)?;
    path.push("cabal.project.local");
    Ok(path)
}

async fn build<P: AsRef<Path>>(component: &str, path: P, cabal: P) -> Result<()> {
    log::info!("Building {component}");
    let path = absolute_ref_path_to_string(&path)?;
    let cabal = absolute_ref_path_to_string(&cabal)?;
    let cmd = format!("cd {path} && {cabal} build all");
    if process_success_inherit(&cmd).await? {
        log::debug!("Successfully built {component}");
        return Ok(());
    }
    Err(anyhow!("Failed building {component}"))
}

pub async fn check_install(component: &str) -> Result<()> {
    if let "cardano-node" = component {
        check_installed_version("cardano-cli").await?;
    }
    let version = check_installed_version(component).await?;
    source_shell().await?;
    log::info!("Successfully installed {component} v{version}");
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
