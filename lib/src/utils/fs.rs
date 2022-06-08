use crate::{async_command, check_env, check_user, get_component_path, print, set_env, spinner, SPINNERS};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::{collections::HashMap, path::Path, thread::sleep, time::Duration};
use tokio::fs::create_dir_all;

pub async fn change_dir(absolute_path: &str) -> Result<()> {
    let cmd = format!("cd {}", absolute_path);
    if async_command(&cmd).await.is_ok() {
        let msg = format!("Changed directory to {}", absolute_path);
        print("green", &msg)
    } else {
        let msg = format!("Failed changing directory to {}", absolute_path);
        print("red", &msg)
    }
}

pub async fn check_dir(absolute_path: &str) -> Result<()> {
    if !Path::new(absolute_path).is_dir() {
        create_dir(absolute_path).await
    } else {
        Ok(())
    }
}

pub async fn check_home_dir() -> Result<String> {
    let user = check_user().await?;
    let home_directory = format!("/home/{}", user.trim());
    set_env("RUNNER_HOME", &home_directory);
    Ok(home_directory)
}

pub async fn check_work_dir() -> Result<String> {
    let home = check_home_dir().await?;
    let install_directory = format!("{}/.cardano", home);
    set_env("WORK_DIR", &install_directory);
    Ok(install_directory)
}

pub async fn copy_binary(component: &str) -> Result<()> {
    let install_dir = check_env("INSTALL_DIR")?;
    let msg = format!("Copying {} binary to {}", component, install_dir);
    print("", &msg)?;
    match component {
        "cardano-node" => copy_node_binaries(&install_dir).await,
        _ => Err(anyhow!("Unknown component")),
    }
}

async fn copy_node_binaries(install_dir: &str) -> Result<()> {
    let component = "cardano-node";
    let path = get_component_path(component).await?;
    let bin_path = format!("{}/scripts/bin-path.sh", path);
    let node = format!("cd {} && cp -p \"$({} cardano-node)\" {}", path, bin_path, install_dir);
    let cli = format!("cd {} && cp -p \"$({} cardano-cli)\" {}", path, bin_path, install_dir);
    let node_bin = format!("{}/cardano-node", install_dir);
    let cli_bin = format!("{}/cardano-cli", install_dir);
    async_command(&node).await?;
    async_command(&cli).await?;
    set_env("CARDANO_NODE_BIN", &node_bin);
    set_env("CARDANO_CLI_BIN", &cli_bin);
    let msg = format!("Successfully copied binaries to {}", install_dir);
    print("green", &msg)
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
    let pb = spinner("Setting up working directory", &SPINNERS);
    let home_dir = check_home_dir().await?;
    let work_dir = check_env("WORK_DIR")?;
    let ipc_dir = format!("{}/ipc", work_dir);
    let config_dir = format!("{}/config", work_dir);
    let data_dir = format!("{}/data", work_dir);
    let libsodium_dir = format!("{}/libsodium", work_dir);
    let mainnet_data_dir = format!("{}/mainnet", data_dir);
    let testnet_data_dir = format!("{}/testnet", data_dir);
    let install_dir = format!("{}/.local/bin", home_dir);
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("working", &work_dir);
    map.insert("ipc", &ipc_dir);
    map.insert("config", &config_dir);
    map.insert("data", &data_dir);
    map.insert("mainnet", &mainnet_data_dir);
    map.insert("testnet", &testnet_data_dir);
    map.insert("install", &install_dir);
    map.insert("libsodium", &libsodium_dir);
    for (key, value) in map.iter() {
        sleep(Duration::from_millis(300));
        check_dir(value).await?;
        let mut env_key = format!("{}-dir", key);
        env_key = env_key.to_case(Case::UpperSnake);
        set_env(&env_key, value);
        pb.set_message(format!("{} directory checked", key));
    }
    chownr(&work_dir).await?;
    pb.finish_and_clear();
    print("green", "Working directory is setup")
}

// TODO: Use standard library instead
pub async fn chownr(absolute_path: &str) -> Result<()> {
    let user = check_user().await?;
    let user = user.trim();
    let cmd = format!("chown -R {}:{} {}", user, user, absolute_path);
    if async_command(&cmd).await.is_ok() {
        Ok(())
    } else {
        Err(anyhow!("Failed adjusting permissions of {}", absolute_path))
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_setup_work_dir() {
        unimplemented!();
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
    #[ignore]
    async fn test_check_work_dir() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_home_dir() {
        unimplemented!();
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
