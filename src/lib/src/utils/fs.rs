use crate::{
    check_env, component_to_string, copy_address_binary, copy_bech32_binary,
    copy_node_binaries, copy_wallet_binary, match_component, read_setting,
    set_env, Component, DIRECTORIES,
};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

pub fn setup_work_dir() -> Result<()> {
    log::debug!("Setting up working directory");
    for key in DIRECTORIES {
        let key = format!("{key}_dir");
        let directory = read_setting(&key)?;
        check_dir(&directory)?;
        let key = key.to_case(Case::UpperSnake);
        set_env(&key, &directory);
    }
    Ok(())
}

pub fn check_dir<P: AsRef<Path>>(absolute_path: P) -> Result<()> {
    let path = path_to_string(absolute_path.as_ref())?;
    log::trace!("Checking {path}");
    if !absolute_path.as_ref().is_dir() {
        log::debug!("{path} is not a directory");
        return create_dir(absolute_path);
    }
    log::trace!("The path {path} exists");
    Ok(())
}

pub fn check_work_dir() -> Result<impl AsRef<Path>> {
    log::debug!("Checking the working directory");
    let path = read_setting("work_dir")?;
    set_env("WORK_DIR", &path);
    Ok(PathBuf::from(&path))
}

pub async fn copy_binary(component: Component) -> Result<()> {
    let component = component_to_string(component);
    log::debug!("Copying the built binaries of {component}");
    let install_dir = check_env("INSTALL_DIR")?;
    match match_component(&component) {
        Component::Node => copy_node_binaries(&install_dir).await,
        Component::Cli => copy_node_binaries(&install_dir).await,
        Component::Wallet => copy_wallet_binary(&install_dir).await,
        Component::Address => copy_address_binary(&install_dir).await,
        Component::Bech32 => copy_bech32_binary(&install_dir).await,
    }
}

pub fn create_dir<P: AsRef<Path>>(absolute_path: P) -> Result<()> {
    create_dir_all(&absolute_path)?;
    let path = absolute_ref_path_to_string(&absolute_path)?;
    log::info!("Created directory: {path}");
    Ok(())
}

pub fn path_to_string(path: &Path) -> Result<String> {
    log::trace!("Parsing the absolute path to a string");
    if let Some(path) = path.to_str() {
        return Ok(path.to_string());
    }
    Err(anyhow!("Failed to parse path to string"))
}

pub fn absolute_ref_path_to_string<P: AsRef<Path>>(
    absolute_path: P,
) -> Result<String> {
    log::trace!("Parsing the path to string if the path is absolute");
    let path = absolute_path.as_ref();
    let parsed = path_to_string(path)?;
    if !path.exists() {
        log::error!("The path {parsed} does not exist");
        return Err(anyhow!("The path {parsed} does not exist"));
    }
    if path.is_absolute() {
        return path_to_string(path);
    }
    log::error!("The path {parsed} is not absolute");
    Err(anyhow!("The path {parsed} is not absolute"))
}

pub fn get_bin_path(bin: &str) -> Result<PathBuf> {
    log::debug!("Getting the path of the binary {bin}");
    if let Some(mut path) = dirs::executable_dir() {
        path.push(bin);
        if !path.exists() {
            return Err(anyhow!("The {bin} binary was not found"));
        }
        let parsed = absolute_ref_path_to_string(&path)?;
        log::debug!("The path to the {bin} binary: {parsed}");
        return Ok(path);
    }
    Err(anyhow!(
        "XDG_BIN_HOME is not set, failed to check if {bin} is installed"
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_setup_work_dir() -> Result<()> {
        setup_work_dir()?;
        for key in DIRECTORIES {
            let key = format!("{key}_dir");
            let setting = read_setting(&key)?;
            let key = key.to_case(Case::UpperSnake);
            let value = check_env(&key)?;
            assert_eq!(value, setting);
        }
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_dir() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_copy_binary() {
        unimplemented!();
    }

    #[tokio::test]
    async fn test_check_work_dir() -> Result<()> {
        let home = dirs::home_dir().unwrap();
        let home = home.to_str().unwrap();
        log::debug!("{home}");
        let work_dir = check_work_dir()?;
        let work_dir = work_dir.as_ref().to_str().unwrap();
        log::debug!("{work_dir}");
        let result = check_env("WORK_DIR")?;
        log::debug!("{result}");
        assert_eq!(work_dir, result);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_dir() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_change_dir() {
        unimplemented!();
    }
}
