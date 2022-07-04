use crate::{
    async_command, async_command_pipe, check_env, get_component_path, read_setting, set_env, CARDANO_NODE_RELEASE_URL, DIRECTORIES,
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

pub async fn copy_binary(component: &str) -> Result<()> {
    log::debug!("Copying the built binaries of {component}");
    let install_dir = check_env("INSTALL_DIR")?;
    if component.eq("cardano-node") {
        copy_node_binaries(&install_dir).await?;
    }
    Ok(())
}

async fn copy_node_binaries<P: AsRef<Path>>(install_dir: P) -> Result<()> {
    let install_dir = absolute_ref_path_to_string(install_dir.as_ref())?;
    let component = "cardano-node";
    let mut path = get_component_path(component)?;
    let parsed_path = absolute_ref_path_to_string(&path)?;
    let bin_path = format!("{parsed_path}/scripts/bin-path.sh");
    path.push("scripts");
    path.push("bin-path.sh");
    let node = format!("cd {parsed_path} && cp -p \"$({bin_path} cardano-node)\" {install_dir}");
    let cli = format!("cd {parsed_path} && cp -p \"$({bin_path} cardano-cli)\" {install_dir}");
    let node_bin = format!("{install_dir}/cardano-node");
    let cli_bin = format!("{install_dir}/cardano-cli");
    log::info!("Copying built cardano-node binary to {node_bin}");
    async_command(&node).await?;
    log::info!("Copying built cardano-cli binary to {cli_bin}");
    async_command(&cli).await?;
    set_env("CARDANO_NODE_BIN", &node_bin);
    set_env("CARDANO_CLI_BIN", &cli_bin);
    Ok(())
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

pub fn absolute_ref_path_to_string<P: AsRef<Path>>(absolute_path: P) -> Result<String> {
    log::trace!("Parsing the path to string if the path is absolute");
    let path = absolute_path.as_ref();
    let parsed = path_to_string(path)?;
    if !path.exists() {
        return Err(anyhow!("The path {parsed} does not exist"));
    }
    if path.is_absolute() {
        return path_to_string(path);
    }
    Err(anyhow!("The path {parsed} is not absolute"))
}

pub fn get_bin_path(bin: &str) -> Result<PathBuf> {
    log::debug!("Getting the path of the binary {bin}");
    if let Some(mut dir) = dirs::executable_dir() {
        dir.push(bin);
        let path = dir;
        let parsed = absolute_ref_path_to_string(&path)?;
        log::debug!("The path to the {bin} binary: {parsed}");
        return Ok(path);
    }
    Err(anyhow!("XDG_DATA_HOME is not set, failed to check if {bin} is installed"))
}

pub async fn is_bin_installed(bin: &str) -> Result<bool> {
    log::debug!("Checking if {bin} is already installed");
    if let Some(mut dir) = dirs::executable_dir() {
        dir.push(bin);
        return Ok(dir.is_file());
    }
    Err(anyhow!("XDG_DATA_HOME is not set, failed to check if {bin} is installed"))
}

pub async fn check_installed_version(component: &str) -> Result<String> {
    log::debug!("Checking installed version of {component}");
    let component_bin_path = get_bin_path(component)?;
    let path = absolute_ref_path_to_string(component_bin_path)?;
    let cmd = format!("{path} --version | awk {} | head -n1", "'{print $2}'");
    let version = async_command_pipe(&cmd).await?;
    let installed_version: String = String::from(version.trim());
    Ok(installed_version)
}

pub async fn check_latest_version(component: &str) -> Result<String> {
    log::debug!("Checking latest {component} version");
    let cmd = format!("curl -s {CARDANO_NODE_RELEASE_URL} | jq -r .tag_name");
    let response = async_command_pipe(&cmd).await?;
    Ok(String::from(response.trim()))
}

pub fn get_db_path() -> Result<()> {
    let mut work_dir = check_work_dir()?.as_ref().to_path_buf();
    work_dir.push("config");
    Ok(())
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
