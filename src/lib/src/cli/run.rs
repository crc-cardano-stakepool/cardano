use crate::{absolute_ref_path_to_string, async_command, path_to_string, read_setting, CONFIG_BASE_URL, CONFIG_FILES};
use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn get_db(network: &str) -> Result<Option<PathBuf>> {
    let key = format!("{network}_db_dir");
    let path = read_setting(&key)?;
    let db = PathBuf::from(&path);
    if !db.exists() {
        log::error!("Invalid db");
        return Err(anyhow!("The path {path} does not exist"));
    }
    if !db.is_dir() {
        log::error!("Invalid db");
        return Err(anyhow!("The path {path} is not a directory"));
    }
    Ok(Some(db))
}
pub fn get_topology(network: &str) -> Result<Option<PathBuf>> {
    let key = format!("{network}_config_dir");
    let path = read_setting(&key)?;
    let mut topology = PathBuf::from(&path);
    let key = format!("{network}-topology.json");
    topology.push(key);
    let path = absolute_ref_path_to_string(&topology)?;
    if !topology.exists() {
        log::error!("Invalid topology");
        return Err(anyhow!("The path {path} does not exist"));
    }
    if !topology.is_file() {
        log::error!("Invalid topology");
        return Err(anyhow!("The path {path} is not a file"));
    }
    Ok(Some(topology))
}
pub fn get_config(network: &str) -> Result<Option<PathBuf>> {
    let key = format!("{network}_config_dir");
    let path = read_setting(&key)?;
    let mut config = PathBuf::from(&path);
    let key = format!("{network}-config.json");
    config.push(key);
    let path = absolute_ref_path_to_string(&config)?;
    if !config.exists() {
        log::error!("Invalid config");
        return Err(anyhow!("The path {path} does not exist"));
    }
    if !config.is_file() {
        log::error!("Invalid config");
        return Err(anyhow!("The path {path} is not a file"));
    }
    Ok(Some(config))
}

pub async fn check_config_files(network: &str) -> Result<()> {
    log::debug!("Checking configuration files");
    let key = format!("{network}_config_dir");
    let path = read_setting(&key)?;
    let db = PathBuf::from(&path);
    if !db.exists() {
        return Err(anyhow!("Configuration directory does not exist"));
    }
    for file in CONFIG_FILES {
        check_config_file(db.clone(), network, file).await?;
    }
    Ok(())
}

pub async fn check_config_file(mut db: PathBuf, network: &str, file: &str) -> Result<()> {
    let download_path = path_to_string(&db)?;
    let name = format!("{network}-{file}.json");
    db.push(&name);
    let file = path_to_string(&db)?;
    log::debug!("Checking config file {file}");
    if !db.exists() {
        log::warn!("Config file {file} not found, downloading it");
        let cmd = format!("wget {CONFIG_BASE_URL}/{name} -P {download_path}");
        async_command(&cmd).await?;
        log::info!("Downloaded config file {file} successfully");
    }
    log::debug!("Config file found");
    db.pop();
    Ok(())
}
