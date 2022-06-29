use crate::{async_command, check_env, check_user, get_component_path, set_env};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::{collections::HashMap, path::Path};
use tokio::fs::create_dir_all;

pub async fn check_dir(absolute_path: &str) -> Result<()> {
    if !Path::new(absolute_path).is_dir() {
        create_dir(absolute_path).await
    } else {
        Ok(())
    }
}

pub fn check_home_dir() -> Result<String> {
    let user = check_user()?;
    let home_directory = format!("/home/{}", user.trim());
    set_env("RUNNER_HOME", &home_directory);
    Ok(home_directory)
}

pub async fn check_work_dir(home: &str) -> Result<String> {
    let install_directory = format!("{home}/.config/.cardano");
    set_env("WORK_DIR", &install_directory);
    Ok(install_directory)
}

pub async fn copy_binary(component: &str) -> Result<()> {
    let install_dir = check_env("INSTALL_DIR")?;
    if component == "cardano-node" {
        copy_node_binaries(&install_dir).await?;
    }
    Ok(())
}

async fn copy_node_binaries(install_dir: &str) -> Result<()> {
    let component = "cardano-node";
    let path = get_component_path(component).await?;
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
    chownr(absolute_path).await
}

pub fn file_exists(absolute_path: &str) -> bool {
    Path::new(absolute_path).exists()
}

pub fn is_dir(absolute_path: &str) -> bool {
    Path::new(absolute_path).is_dir()
}

pub async fn setup_work_dir() -> Result<()> {
    let home_dir = check_home_dir()?;
    check_work_dir(&home_dir).await?;
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
    chownr(&work_dir).await?;
    Ok(())
}

// TODO: Use standard library instead
pub async fn chownr(absolute_path: &str) -> Result<()> {
    let user = check_user()?;
    let user = user.trim();
    let cmd = format!("chown -R {user}:{user} {absolute_path}");
    if async_command(&cmd).await.is_ok() {
        Ok(())
    } else {
        Err(anyhow!("Failed adjusting permissions of {absolute_path}"))
    }
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
        let home = check_home_dir()?;
        let work_dir = check_work_dir(&home).await?;
        let result = check_env("WORK_DIR")?;
        assert_eq!(work_dir, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_check_home_dir() -> Result<()> {
        let home = check_home_dir()?;
        let result = check_env("RUNNER_HOME")?;
        assert_eq!(home, result);
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

    #[tokio::test]
    #[ignore]
    async fn test_chownr() {
        unimplemented!();
    }
}
