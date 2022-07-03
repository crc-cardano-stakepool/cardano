use crate::{async_command, async_command_pipe, check_env, check_user, get_component_path, set_env, CARDANO_NODE_RELEASE_URL};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::fs::create_dir_all;

pub async fn check_dir(absolute_path: &str) -> Result<()> {
    if !Path::new(absolute_path).is_dir() {
        create_dir(absolute_path).await
    } else {
        Ok(())
    }
}

pub fn check_work_dir() -> Result<PathBuf> {
    let mut work_dir = dirs::config_dir()
        .ok_or_else(|| anyhow!("Failed to determine XDG_DATA_CONFIG"))
        .unwrap();
    work_dir.push(".cardano");
    if let Some(path) = work_dir.to_str() {
        set_env("WORK_DIR", path);
        Ok(work_dir)
    } else {
        Err(anyhow!("Failed to set working directory"))
    }
}

pub async fn copy_binary(component: &str) -> Result<()> {
    log::info!("Copying the built binaries of {component}");
    let install_dir = check_env("INSTALL_DIR")?;
    if component == "cardano-node" {
        copy_node_binaries(&install_dir).await?;
    }
    Ok(())
}

async fn copy_node_binaries(install_dir: &str) -> Result<()> {
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

pub async fn create_dir(absolute_path: &str) -> Result<()> {
    create_dir_all(absolute_path).await?;
    Ok(())
}

pub fn file_exists(absolute_path: &str) -> bool {
    Path::new(absolute_path).exists()
}

pub fn is_dir(absolute_path: &str) -> bool {
    Path::new(absolute_path).is_dir()
}

pub async fn setup_work_dir() -> Result<()> {
    log::info!("Setting up working directory");
    let home_dir = dirs::home_dir().expect("Failed to read $HOME");
    let home_dir = home_dir.to_str().expect("Failed to parse $HOME to string");
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
    let install_dir = format!("{home_dir}/.local/bin");
    let map: HashMap<&str, &String> = HashMap::from([
        ("working", &work_dir),
        ("ipc", &ipc_dir),
        ("cardano", &cardano_dir),
        ("config", &config_dir),
        ("data", &data_dir),
        ("mainnet", &mainnet_data_dir),
        ("testnet", &testnet_data_dir),
        ("install", &install_dir),
        ("libsodium", &libsodium_dir),
        ("secp256k1", &secp256k1_dir),
    ]);
    for (key, value) in map.iter() {
        check_dir(value).await?;
        let mut env_key = format!("{key}-dir");
        env_key = env_key.to_case(Case::UpperSnake);
        set_env(&env_key, value);
    }
    Ok(())
}

pub fn get_bin_path(bin: &str) -> Result<String> {
    let user = check_user()?;
    let path = format!("/home/{user}/.local/bin/{bin}");
    Ok(path)
}

pub async fn is_bin_installed(bin: &str) -> Result<bool> {
    log::debug!("Checking if {bin} is already installed");
    let user = check_user()?;
    let file = format!("/home/{user}/.local/bin/{bin}");
    Ok(file_exists(&file))
}

pub async fn check_installed_version(component: &str) -> Result<String> {
    log::info!("Checking installed version of {component}");
    let component_bin_path = get_bin_path(component)?;
    let cmd = format!("{component_bin_path} --version | awk {} | head -n1", "'{print $2}'");
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

    #[tokio::test]
    async fn test_setup_work_dir() -> Result<()> {
        setup_work_dir().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_is_dir() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_file_exists() {
        unimplemented!();
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
        let work_dir = work_dir.to_str().unwrap();
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
