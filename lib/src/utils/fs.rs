use crate::{async_command, check_env, check_user, get_component_path, print, set_env, spinner, SPINNERS};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::{collections::HashMap, path::Path, thread::sleep, time::Duration};
use tokio::fs::create_dir_all;

pub async fn change_dir(absolute_path: &str) -> Result<()> {
    let cmd = format!("cd {absolute_path}");
    if async_command(&cmd).await.is_ok() {
        let msg = format!("Changed directory to {absolute_path}");
        print("green", &msg)
    } else {
        let msg = format!("Failed changing directory to {absolute_path}");
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
    let install_directory = format!("{home}/.cardano");
    set_env("WORK_DIR", &install_directory);
    Ok(install_directory)
}

pub async fn copy_binary(component: &str) -> Result<()> {
    let install_dir = check_env("INSTALL_DIR")?;
    let msg = format!("Copying {component} binary to {install_dir}");
    print("", &msg)?;
    match component {
        "cardano-node" => copy_node_binaries(&install_dir).await,
        _ => Err(anyhow!("Unknown component {component}")),
    }
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
    let msg = format!("Successfully copied binaries to {install_dir}");
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
    let ipc_dir = format!("{work_dir}/ipc");
    let config_dir = format!("{work_dir}/config");
    let data_dir = format!("{work_dir}/data");
    let libsodium_dir = format!("{work_dir}/libsodium");
    let mainnet_data_dir = format!("{data_dir}/mainnet");
    let testnet_data_dir = format!("{data_dir}/testnet");
    let install_dir = format!("{home_dir}/.local/bin");
    let map: HashMap<&str, &String> = HashMap::from([
        ("working", &work_dir),
        ("ipc", &ipc_dir),
        ("config", &config_dir),
        ("data", &data_dir),
        ("mainnet", &mainnet_data_dir),
        ("testnet", &testnet_data_dir),
        ("install", &install_dir),
        ("libsodium", &libsodium_dir),
    ]);
    for (key, value) in map.iter() {
        sleep(Duration::from_millis(300));
        check_dir(value).await?;
        let mut env_key = format!("{key}-dir");
        env_key = env_key.to_case(Case::UpperSnake);
        set_env(&env_key, value);
        pb.set_message(format!("{key} directory checked"));
    }
    chownr(&work_dir).await?;
    pb.finish_and_clear();
    print("green", "Working directory is setup")
}

// TODO: Use standard library instead
pub async fn chownr(absolute_path: &str) -> Result<()> {
    let user = check_user().await?;
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
