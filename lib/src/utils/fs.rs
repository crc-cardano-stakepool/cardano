use crate::{async_command, check_env, check_user, get_component_path, print, set_env};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use nix::unistd::{getgid, getuid};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::fs::create_dir_all;

pub async fn check_dir(absolute_path: impl AsRef<Path>) -> Result<()> {
    if !absolute_path.as_ref().is_dir() {
        create_dir(absolute_path).await
    } else {
        Ok(())
    }
}

pub async fn check_home_dir() -> Result<PathBuf> {
    let user = check_user().await?;
    let mut home = PathBuf::from("/home");
    home.push(user);
    if let Some(path) = home.to_str() {
        set_env("RUNNER_HOME", path);
    }
    Ok(home)
}

pub async fn check_work_dir(home: impl AsRef<Path>) -> Result<PathBuf> {
    let path = home.as_ref();
    let mut work_dir = PathBuf::from(path);
    work_dir.push(".cardano");
    if let Some(work) = work_dir.to_str() {
        set_env("WORK_DIR", work);
    }
    Ok(work_dir)
}

pub async fn copy_binary(component: &str) -> Result<()> {
    let install_dir = check_env("INSTALL_DIR")?;
    let path = Path::new(&install_dir);
    let msg = format!("Copying {component} binary to {install_dir}");
    print("", &msg)?;
    match component {
        "cardano-node" => copy_node_binaries(path).await,
        _ => Err(anyhow!("Unknown component {component}")),
    }
}

pub fn get_path_string(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    if let Some(path) = path.to_str() {
        Ok(path.to_string())
    } else {
        Err(anyhow!("Failed converting path to string"))
    }
}

async fn copy_node_binaries(install_dir: impl AsRef<Path>) -> Result<()> {
    let install_dir = get_path_string(install_dir)?;
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

pub async fn create_dir(absolute_path: impl AsRef<Path>) -> Result<()> {
    create_dir_all(&absolute_path).await?;
    chownr(&absolute_path)?;
    Ok(())
}

pub async fn setup_work_dir() -> Result<()> {
    let home_dir = check_home_dir().await?;
    let work_dir = check_work_dir(&home_dir).await?;
    let mut ipc_dir = PathBuf::from(&work_dir);
    ipc_dir.push("ipc");
    let mut cardano_dir = PathBuf::from(&work_dir);
    cardano_dir.push("cardano");
    let mut config_dir = PathBuf::from(&work_dir);
    config_dir.push("config");
    let mut data_dir = PathBuf::from(&work_dir);
    data_dir.push("data");
    let mut libsodium_dir = PathBuf::from(&work_dir);
    libsodium_dir.push("libsodium");
    let mut secp256k1_dir = PathBuf::from(&work_dir);
    secp256k1_dir.push("secp256k1");
    let mut mainnet_data_dir = PathBuf::from(&data_dir);
    mainnet_data_dir.push("mainnet");
    let mut testnet_data_dir = PathBuf::from(&data_dir);
    testnet_data_dir.push("testnet");
    let mut install_dir = PathBuf::from(&home_dir);
    install_dir.push(".local");
    install_dir.push("bin");
    let map: HashMap<&str, PathBuf> = HashMap::from([
        ("working", work_dir),
        ("ipc", ipc_dir),
        ("cardano", cardano_dir),
        ("config", config_dir),
        ("data", data_dir),
        ("mainnet", mainnet_data_dir),
        ("testnet", testnet_data_dir),
        ("libsodium", libsodium_dir),
        ("secp256k1", secp256k1_dir),
        ("install", install_dir),
    ]);
    for (key, value) in map.iter() {
        check_dir(value).await?;
        let mut env_key = format!("{key}-dir");
        env_key = env_key.to_case(Case::UpperSnake);
        if let Some(path) = value.to_str() {
            set_env(&env_key, path);
        }
    }
    print("green", "Working directory is setup")
}

pub fn chownr(path: impl AsRef<Path>) -> Result<()> {
    let uid = Some(getuid());
    let gid = Some(getgid());
    let path = path.as_ref();
    if path.is_dir() {
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            chownr(entry.path().as_path())?;
        }
    }
    nix::unistd::chown(path, uid, gid)?;
    Ok(())
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
        let home = check_home_dir().await?;
        let work_dir = check_work_dir(&home).await?;
        let work_dir = get_path_string(work_dir)?;
        let result = check_env("WORK_DIR")?;
        assert_eq!(work_dir, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_check_home_dir() -> Result<()> {
        let home = check_home_dir().await?;
        let home = get_path_string(home)?;
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
