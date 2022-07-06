use crate::{
    async_command, build, check_env, check_install, check_installed_version,
    check_latest_version, check_project_file, clone_component, configure_build,
    copy_binary, get_component_path, get_ghc_version, get_project_file,
    is_component_installed, path_to_string, proceed, set_component_dir,
    set_confirm, setup_address, update_cabal, Component, ShellConfig,
};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub async fn check_latest_address(confirm: bool) -> Result<()> {
    if !is_component_installed(Component::Address)? {
        return install_latest_address(confirm).await;
    }
    let installed = check_installed_version(Component::Address).await?;
    let latest = check_latest_version(Component::Address).await?;
    if installed.eq(&latest) {
        log::info!(
            "The cardano-address v{installed} binary is already up to date!"
        );
        return Ok(());
    }
    install_latest_address(confirm).await
}

pub async fn install_latest_address(confirm: bool) -> Result<()> {
    set_confirm(confirm);
    setup_address().await?;
    let version = check_latest_version(Component::Address).await?;
    let msg = format!(
        "Do you want to install the latest cardano-address v{version} binary?"
    );
    if !confirm && proceed(&msg)? {
        return install_address().await;
    }
    install_address().await
}

pub async fn install_address() -> Result<()> {
    build_address().await?;
    copy_binary(Component::Address).await?;
    check_install(Component::Address).await?;
    ShellConfig::source_shell().await
}

pub async fn build_address() -> Result<()> {
    log::info!("Building cardano-address");
    clone_component(Component::Address).await?;
    let ghc_version = get_ghc_version().await?;
    let cabal = check_env("CABAL_BIN")?;
    let cabal = PathBuf::from(&cabal);
    let project_file = get_project_file(Component::Address)?;
    let path = get_component_path(Component::Address)?;
    update_cabal(&path, &cabal).await?;
    check_project_file(&project_file).await?;
    configure_build(&ghc_version, &path, &cabal).await?;
    build(Component::Address, &path, &cabal).await
}

pub async fn copy_address_binary<P: AsRef<Path>>(install_dir: P) -> Result<()> {
    let install_dir = path_to_string(install_dir.as_ref())?;
    log::info!("Installing the built cardano-address binary to {install_dir}");
    let path = set_component_dir(Component::Address)?;
    let cmd = format!(
        "cd {path} && \
            cabal install cardano-addresses-cli \
            --install-method=copy \
            --overwrite-policy=always \
            --installdir={install_dir}"
    );
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_build_address() -> Result<()> {
        build_address().await
    }
}
