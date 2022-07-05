use crate::{
    async_command, build, check_env, check_install, check_installed_version, check_latest_version, check_project_file, clone_component,
    configure_build, copy_binary, get_component_dir, get_component_path, get_ghc_version, get_project_file, is_component_installed,
    path_to_string, proceed, set_confirm, setup_wallet, update_cabal, Component, ShellConfig,
};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub async fn check_latest_wallet(confirm: bool) -> Result<()> {
    if !is_component_installed(Component::Wallet)? {
        return install_latest_wallet(confirm).await;
    }
    let installed = check_installed_version(Component::Wallet).await?;
    let latest = check_latest_version(Component::Wallet).await?;
    if installed.eq(&latest) {
        log::info!("The cardano-wallet {installed} is already up to date!");
        return Ok(());
    }
    install_latest_wallet(confirm).await
}

pub async fn install_latest_wallet(confirm: bool) -> Result<()> {
    set_confirm(confirm);
    setup_wallet().await?;
    let version = check_latest_version(Component::Wallet).await?;
    let msg = format!("Do you want to install the latest cardano-wallet {version} binary?");
    if !confirm && proceed(&msg)? {
        return install_wallet().await;
    }
    install_wallet().await
}

pub async fn install_wallet() -> Result<()> {
    build_wallet().await?;
    copy_binary(Component::Wallet).await?;
    check_install(Component::Wallet).await?;
    ShellConfig::source_shell().await
}

pub async fn build_wallet() -> Result<()> {
    log::info!("Building cardano-wallet");
    clone_component(Component::Wallet).await?;
    let ghc_version = get_ghc_version().await?;
    let cabal = check_env("CABAL_BIN")?;
    let cabal = PathBuf::from(&cabal);
    let project_file = get_project_file(Component::Wallet)?;
    let path = get_component_path(Component::Wallet)?;
    update_cabal(&path, &cabal).await?;
    check_project_file(&project_file).await?;
    configure_build(&ghc_version, &path, &cabal).await?;
    build(Component::Wallet, &path, &cabal).await
}

pub async fn copy_wallet_binary<P: AsRef<Path>>(install_dir: P) -> Result<()> {
    let install_dir = path_to_string(install_dir.as_ref())?;
    log::info!("Installing the built cardano-wallet binary to {install_dir}");
    let path = get_component_dir(Component::Wallet)?;
    let cmd =
        format!("cd {path} && cabal install cardano-wallet --install-method=copy --overwrite-policy=always --installdir={install_dir}");
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_build_wallet() -> Result<()> {
        build_wallet().await
    }
}
