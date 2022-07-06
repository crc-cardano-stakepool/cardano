use crate::{
    async_command, build, check_env, check_install, check_installed_version,
    check_latest_version, check_project_file, clone_component, configure_build,
    copy_binary, get_component_path, get_ghc_version, get_project_file,
    is_component_installed, path_to_string, proceed, set_component_dir,
    set_confirm, setup_bech32, update_cabal, Component, ShellConfig,
};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub async fn check_latest_bech32(confirm: bool) -> Result<()> {
    if !is_component_installed(Component::Bech32)? {
        return install_latest_bech32(confirm).await;
    }
    let installed = check_installed_version(Component::Bech32).await?;
    let latest = check_latest_version(Component::Bech32).await?;
    if installed.eq(&latest) {
        log::info!("The bech32 v{installed} binary is already up to date!");
        return Ok(());
    }
    install_latest_bech32(confirm).await
}

pub async fn install_latest_bech32(confirm: bool) -> Result<()> {
    set_confirm(confirm);
    setup_bech32().await?;
    let version = check_latest_version(Component::Bech32).await?;
    let msg =
        format!("Do you want to install the latest bech32 {version} binary?");
    if !confirm && proceed(&msg)? {
        return install_bech32().await;
    }
    install_bech32().await
}

pub async fn install_bech32() -> Result<()> {
    build_bech32().await?;
    copy_binary(Component::Bech32).await?;
    check_install(Component::Bech32).await?;
    ShellConfig::source_shell().await
}

pub async fn build_bech32() -> Result<()> {
    log::info!("Building bech32");
    clone_component(Component::Bech32).await?;
    let ghc_version = get_ghc_version().await?;
    let cabal = check_env("CABAL_BIN")?;
    let cabal = PathBuf::from(&cabal);
    let project_file = get_project_file(Component::Bech32)?;
    let path = get_component_path(Component::Bech32)?;
    update_cabal(&path, &cabal).await?;
    check_project_file(&project_file).await?;
    configure_build(&ghc_version, &path, &cabal).await?;
    build(Component::Bech32, &path, &cabal).await
}

pub async fn copy_bech32_binary<P: AsRef<Path>>(install_dir: P) -> Result<()> {
    let install_dir = path_to_string(install_dir.as_ref())?;
    log::info!("Installing the built bech32 binary to {install_dir}");
    let path = set_component_dir(Component::Bech32)?;
    let cmd = format!(
        "cd {path} && \
            cabal install bech32 \
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
    async fn test_build_bech32() -> Result<()> {
        build_bech32().await
    }
}
