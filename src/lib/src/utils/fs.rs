use crate::{async_command, async_command_pipe, check_env, get_component_path, set_env, CARDANO_NODE_RELEASE_URL};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::{
    collections::HashMap,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

pub fn setup_work_dir() -> Result<()> {
    log::info!("Setting up working directory");
    check_work_dir()?;
    let work_dir = check_env("WORK_DIR")?;
    let ipc_dir = format!("{work_dir}/ipc");
    let cardano_dir = format!("{work_dir}/cardano");
    let config_dir = format!("{work_dir}/config");
    let data_dir = format!("{work_dir}/data");
    let libsodium_dir = format!("{work_dir}/libsodium");
    let secp256k1_dir = format!("{work_dir}/secp256k1");
    let mainnet_data_dir = format!("{data_dir}/mainnet");
    let testnet_data_dir = format!("{data_dir}/testnet");
    let install_dir = dirs::executable_dir().expect("Failed to read $XDG_BIN_HOME");
    let install_dir = install_dir.to_str().expect("Failed to parse $XDG_BIN_HOME to string").to_string();
    let map: HashMap<&str, &String> = HashMap::from([
        ("work", &work_dir),
        ("ipc", &ipc_dir),
        ("cardano", &cardano_dir),
        ("config", &config_dir),
        ("data", &data_dir),
        ("mainnet", &mainnet_data_dir),
        ("testnet", &testnet_data_dir),
        ("libsodium", &libsodium_dir),
        ("secp256k1", &secp256k1_dir),
        ("install", &install_dir),
    ]);
    for (key, value) in map.iter() {
        check_dir(value)?;
        let mut env_key = format!("{key}-dir");
        env_key = env_key.to_case(Case::UpperSnake);
        set_env(&env_key, value);
    }
    Ok(())
}

pub fn check_dir<P: AsRef<Path>>(absolute_path: P) -> Result<()> {
    let path = path_to_string(absolute_path.as_ref())?;
    log::info!("Checking {path}");
    if !absolute_path.as_ref().is_dir() {
        log::debug!("{path} is not a directory");
        return create_dir(absolute_path);
    }
    Ok(())
}

pub fn check_work_dir() -> Result<impl AsRef<Path>> {
    let mut work_dir = dirs::config_dir()
        .ok_or_else(|| anyhow!("Failed to determine XDG_CONFIG_HOME"))
        .unwrap();
    work_dir.push(".cardano");
    if let Some(path) = work_dir.to_str() {
        set_env("WORK_DIR", path);
        return Ok(work_dir);
    }
    Err(anyhow!("Failed to set working directory"))
}

pub async fn copy_binary(component: &str) -> Result<()> {
    log::info!("Copying the built binaries of {component}");
    let install_dir = check_env("INSTALL_DIR")?;
    if component.eq("cardano-node") {
        copy_node_binaries(&install_dir).await?;
    }
    Ok(())
}

async fn copy_node_binaries<P: AsRef<Path>>(install_dir: P) -> Result<()> {
    let install_dir = absolute_ref_path_to_string(install_dir.as_ref())?;
    log::info!("Copying to {install_dir}");
    let component = "cardano-node";
    let path = get_component_path(component)?;
    let bin_path = format!("{path}/scripts/bin-path.sh");
    let node = format!("cd {path} && cp -p \"$({bin_path} cardano-node)\" {install_dir}");
    let cli = format!("cd {path} && cp -p \"$({bin_path} cardano-cli)\" {install_dir}");
    let node_bin = format!("{install_dir}/cardano-node");
    let cli_bin = format!("{install_dir}/cardano-cli");
    async_command(&node).await?;
    async_command(&cli).await?;
    set_env("CARDANO_NODE_BIN", &node_bin);
    set_env("CARDANO_CLI_BIN", &cli_bin);
    Ok(())
}

pub fn create_dir<P: AsRef<Path>>(absolute_path: P) -> Result<()> {
    let path = absolute_ref_path_to_string(&absolute_path)?;
    log::info!("Creating directory: {path}");
    create_dir_all(absolute_path)?;
    Ok(())
}

pub fn path_to_string(path: &Path) -> Result<String> {
    if let Some(path) = path.to_str() {
        return Ok(path.to_string());
    }
    Err(anyhow!("Failed to parse path to string"))
}

pub fn absolute_ref_path_to_string<P: AsRef<Path>>(absolute_path: P) -> Result<String> {
    let path = absolute_path.as_ref();
    if path.is_absolute() {
        return path_to_string(path);
    }
    let path = path_to_string(path)?;
    Err(anyhow!("The path {path} is not absolute"))
}
pub fn get_bin_path(bin: &str) -> Result<PathBuf> {
    let bin = absolute_ref_path_to_string(bin)?;
    if let Some(mut dir) = dirs::executable_dir() {
        dir.push(bin);
        let path = dir;
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
    log::info!("Checking installed version of {component}");
    let component_bin_path = get_bin_path(component)?;
    let path = absolute_ref_path_to_string(component_bin_path)?;
    let cmd = format!("{path} --version | awk {} | head -n1", "'{print $2}'");
    let version = async_command_pipe(&cmd).await?;
    let installed_version: String = String::from(version.trim());
    Ok(installed_version)
}

pub async fn check_latest_version(component: &str) -> Result<String> {
    log::info!("Checking latest {component} version");
    let cmd = format!("curl -s {CARDANO_NODE_RELEASE_URL} | jq -r .tag_name");
    log::debug!("Executing command: {cmd}");
    let response = async_command_pipe(&cmd).await?;
    log::debug!("Response: {response}");
    Ok(String::from(response.trim()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_setup_work_dir() -> Result<()> {
        setup_work_dir()?;
        let result = check_work_dir()?;
        let result = result.as_ref().to_str().unwrap();
        let work_dir = check_env("WORK_DIR")?;
        assert_eq!(result, work_dir);
        let ipc_dir = format!("{work_dir}/ipc");
        let result = check_env("IPC_DIR")?;
        assert_eq!(result, ipc_dir);
        let cardano_dir = format!("{work_dir}/cardano");
        let result = check_env("CARDANO_DIR")?;
        assert_eq!(result, cardano_dir);
        let config_dir = format!("{work_dir}/config");
        let result = check_env("CONFIG_DIR")?;
        assert_eq!(result, config_dir);
        let data_dir = format!("{work_dir}/data");
        let result = check_env("DATA_DIR")?;
        assert_eq!(result, data_dir);
        let libsodium_dir = format!("{work_dir}/libsodium");
        let result = check_env("LIBSODIUM_DIR")?;
        assert_eq!(result, libsodium_dir);
        let secp256k1_dir = format!("{work_dir}/secp256k1");
        let result = check_env("SECP_256_K_1_DIR")?;
        assert_eq!(result, secp256k1_dir);
        let mainnet_data_dir = format!("{data_dir}/mainnet");
        let result = check_env("MAINNET_DIR")?;
        assert_eq!(result, mainnet_data_dir);
        let testnet_data_dir = format!("{data_dir}/testnet");
        let result = check_env("TESTNET_DIR")?;
        assert_eq!(result, testnet_data_dir);
        let install_dir = dirs::executable_dir().expect("Failed to read $XDG_BIN_HOME");
        let install_dir = install_dir.to_str().expect("Failed to parse $XDG_BIN_HOME to string");
        let result = check_env("INSTALL_DIR")?;
        assert_eq!(result, install_dir);
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
