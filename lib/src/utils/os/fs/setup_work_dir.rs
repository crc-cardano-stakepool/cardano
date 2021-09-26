use crate::{check_dir, check_env};
use anyhow::Result;

pub async fn setup_work_dir() -> Result<()> {
    println!("Setting up working directory");
    let work_dir = check_env("WORK_DIR")?;
    let ipc_dir = format!("{}/ipc", work_dir);
    let config_dir = format!("{}/config", work_dir);
    let data_dir = format!("{}/data/db", work_dir);
    let mainnet_data_dir = format!("{}/mainnet", data_dir);
    let testnet_data_dir = format!("{}/testnet", data_dir);
    check_dir("working", &work_dir).await?;
    check_dir("ipc", &ipc_dir).await?;
    check_dir("config", &config_dir).await?;
    check_dir("data", &data_dir).await?;
    check_dir("mainnet", &mainnet_data_dir).await?;
    check_dir("testnet", &testnet_data_dir).await?;
    Ok(())
}
