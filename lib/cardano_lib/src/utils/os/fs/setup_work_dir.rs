use crate::{check_dir, check_env, chownr, print, set_env, spinner, SPINNERS};
use anyhow::Result;
use convert_case::{Case, Casing};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

pub async fn setup_work_dir() -> Result<()> {
    if let Some(arrows) = SPINNERS.get("arrows") {
        let pb = spinner("Setting up working directory", arrows);
        let mut map: HashMap<&str, &str> = HashMap::new();
        let work_dir = check_env("WORK_DIR")?;
        let ipc_dir = format!("{}/ipc", work_dir);
        let config_dir = format!("{}/config", work_dir);
        let data_dir = format!("{}/data", work_dir);
        let mainnet_data_dir = format!("{}/mainnet", data_dir);
        let testnet_data_dir = format!("{}/testnet", data_dir);
        let install_dir = format!("{}/.local/bin", work_dir);
        map.insert("working", &work_dir);
        map.insert("ipc", &ipc_dir);
        map.insert("config", &config_dir);
        map.insert("data", &data_dir);
        map.insert("mainnet", &mainnet_data_dir);
        map.insert("testnet", &testnet_data_dir);
        map.insert("install", &install_dir);
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
        print("green", "Working directory is setup")?;
    }

    Ok(())
}
