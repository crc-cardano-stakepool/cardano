use crate::{
    absolute_ref_path_to_string, async_command, check_env, check_install, check_installed_version, check_latest_version, clone_component,
    component_to_string, copy_binary, get_component_path, get_ghc_version, is_component_installed, match_component, path_to_string,
    proceed, process_success_inherit, set_confirm, set_env, setup_node, update_cabal, Component, ShellConfig,
};
use anyhow::{anyhow, Result};
use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub async fn check_latest_node(confirm: bool) -> Result<()> {
    if !is_component_installed(Component::Node)? {
        return install_latest_node(confirm).await;
    }
    let installed = check_installed_version(Component::Node).await?;
    let latest = check_latest_version(Component::Node).await?;
    if installed.eq(&latest) {
        log::info!("The cardano-node binary v{installed} is already up to date!");
        return Ok(());
    }
    install_latest_node(confirm).await
}

pub async fn install_latest_node(confirm: bool) -> Result<()> {
    set_confirm(confirm);
    setup_node().await?;
    let version = check_latest_version(Component::Node).await?;
    let msg = format!("Do you want to install the latest cardano-node v{version} binary?");
    if !confirm && proceed(&msg)? {
        return install_node().await;
    }
    install_node().await
}

pub async fn install_node() -> Result<()> {
    build_node().await?;
    copy_binary(Component::Node).await?;
    check_install(Component::Node).await?;
    ShellConfig::source_shell().await
}

pub async fn build_node() -> Result<()> {
    log::info!("Building cardano-node");
    clone_component(Component::Node).await?;
    let ghc_version = get_ghc_version().await?;
    let cabal = check_env("CABAL_BIN")?;
    let cabal = PathBuf::from(&cabal);
    let project_file = get_project_file(Component::Node)?;
    let path = get_component_path(Component::Node)?;
    update_cabal(&path, &cabal).await?;
    check_project_file(&project_file).await?;
    configure_build(&ghc_version, &path, &cabal).await?;
    update_project_file(&project_file)?;
    build(Component::Node, &path, &cabal).await
}

pub async fn check_project_file<P: AsRef<Path>>(project_file: P) -> Result<()> {
    log::debug!("Checking if the project file already exists");
    let file = project_file.as_ref();
    let path = path_to_string(file)?;
    log::debug!("Project file: {path}");
    let file_name = file.file_name().unwrap().to_str().unwrap();
    log::debug!("File name: {file_name}");
    if !file.is_file() {
        log::debug!("Project file {path} is not a file");
        return Ok(());
    }
    if file_name.eq("cabal.project.local") {
        log::warn!("Project file already exists, removing it");
        let cmd = format!("rm {path}");
        async_command(&cmd).await?;
        return Ok(());
    }
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

pub fn update_project_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let file_path = absolute_ref_path_to_string(&path)?;
    if !path.as_ref().is_file() {
        return Err(anyhow!("The path {file_path} is not a file"));
    }
    log::info!("Updating the project file at {file_path}");
    let mut f = std::fs::File::options()
        .write(true)
        .append(true)
        .open(path)
        .map_err(|err| anyhow!("Failed to open {file_path}: {err}"))
        .unwrap();
    let value = "package cardano-crypto-praos\n  flags: -external-libsodium-vrf\n";
    writeln!(f, "{value}")
        .map_err(|err| anyhow!("Failed to write {value} to {file_path}: {err}"))
        .unwrap();
    Ok(())
}

pub fn get_project_file(component: Component) -> Result<PathBuf> {
    let component = component_to_string(component);
    log::debug!("Getting the project file of the {component} source reposity");
    let mut path = get_component_path(match_component(&component))?;
    path.push("cabal.project.local");
    Ok(path)
}

pub async fn build<P: AsRef<Path>>(component: Component, path: P, cabal: P) -> Result<()> {
    let component = component_to_string(component);
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

pub async fn copy_node_binaries<P: AsRef<Path>>(install_dir: P) -> Result<()> {
    let install_dir = absolute_ref_path_to_string(install_dir.as_ref())?;
    let mut path = get_component_path(Component::Node)?;
    let parsed_path = absolute_ref_path_to_string(&path)?;
    let bin_path = format!("{parsed_path}/scripts/bin-path.sh");
    path.push("scripts");
    path.push("bin-path.sh");
    let components = ["cardano-node", "cardano-cli"];
    for component in components {
        let cmd = format!("cd {parsed_path} && cp -p \"$({bin_path} {component})\" {install_dir}");
        let path = format!("{install_dir}/{component}");
        if component.eq("cardano-node") {
            set_env("CARDANO_NODE_BIN", &path);
        } else {
            set_env("CARDANO_CLI_BIN", &path);
        }
        log::info!("Copying built {component} binary to {path}");
        async_command(&cmd).await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::{Read, Seek, SeekFrom};
    use tempfile::NamedTempFile;

    use crate::set_component_dir;

    use super::*;

    #[test]
    fn test_update_project_file() {
        let mut project_file = NamedTempFile::new().unwrap();
        update_project_file(&project_file).unwrap();
        project_file.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        project_file.read_to_string(&mut buf).unwrap();
        let expected = format!("package cardano-crypto-praos\n  flags: -external-libsodium-vrf\n");
        assert_eq!(expected, buf);
    }

    #[test]
    fn test_get_project_file() {
        let component = Component::Node;
        set_component_dir(component).unwrap();
        let mut path = get_component_path(component).unwrap();
        path.push("cabal.project.local");
        let project_file = get_project_file(component).unwrap();
        assert_eq!(path, project_file)
    }

    #[tokio::test]
    async fn test_check_project_file() {
        let file_name = "cabal.project.local";
        let dir = tempfile::Builder::new().tempdir().unwrap();
        let file_path = dir.path().join(file_name);
        let project_file_name = file_path.file_name().unwrap().to_str().unwrap();
        std::fs::File::create(&file_path).unwrap();
        assert_eq!(file_name, project_file_name);
        check_project_file(&file_path).await.unwrap();
        assert!(!file_path.exists());
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
