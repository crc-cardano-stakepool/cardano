use crate::CARDANO_CONFIG_FILE_NAME;
use anyhow::{anyhow, Result};
use config::Config;
use serde::Serialize;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf, sync::RwLock};

#[derive(Hash, Serialize, Debug, Eq, PartialEq)]
pub struct Settings {
    pub work_dir: PathBuf,
    pub log_file: PathBuf,
    pub ipc_dir: PathBuf,
    pub node_socket_path: PathBuf,
    pub cardano_dir: PathBuf,
    pub config_dir: PathBuf,
    pub mainnet_config_dir: PathBuf,
    pub testnet_config_dir: PathBuf,
    pub mainnet_db_dir: PathBuf,
    pub testnet_db_dir: PathBuf,
    pub libsodium_dir: PathBuf,
    pub secp256k1_dir: PathBuf,
    pub install_dir: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        log::debug!("Creating default settings");
        let mut work_dir = dirs::config_dir().expect("Failed to read XDG_CONFIG_HOME");
        work_dir.push(".cardano");
        let mut log_file = PathBuf::from(&work_dir);
        log_file.push("logs");
        let mut ipc_dir = PathBuf::from(&work_dir);
        ipc_dir.push("ipc");
        let mut node_socket_path = PathBuf::from(&ipc_dir);
        node_socket_path.push("node.socket");
        let mut cardano_dir = PathBuf::from(&work_dir);
        cardano_dir.push("cardano");
        let mut config_dir = PathBuf::from(&work_dir);
        config_dir.push("config");
        let mut mainnet_config_dir = PathBuf::from(&config_dir);
        mainnet_config_dir.push("mainnet");
        let mut testnet_config_dir = PathBuf::from(&config_dir);
        testnet_config_dir.push("testnet");
        let mut mainnet_db_dir = PathBuf::from(&mainnet_config_dir);
        mainnet_db_dir.push("db");
        let mut testnet_db_dir = PathBuf::from(&testnet_config_dir);
        testnet_db_dir.push("db");
        let mut libsodium_dir = PathBuf::from(&work_dir);
        libsodium_dir.push("libsodium");
        let mut secp256k1_dir = PathBuf::from(&work_dir);
        secp256k1_dir.push("secp256k1");
        let install_dir = dirs::executable_dir().expect("Read XDG_BIN_HOME");
        Self {
            work_dir,
            log_file,
            ipc_dir,
            node_socket_path,
            cardano_dir,
            config_dir,
            mainnet_config_dir,
            testnet_config_dir,
            mainnet_db_dir,
            testnet_db_dir,
            libsodium_dir,
            secp256k1_dir,
            install_dir,
        }
    }
}

lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let settings = Settings::default();
        log::debug!("Serializing settings to toml");
        let toml = toml::to_string(&settings)
            .map_err(|err| anyhow!("Failed to serialize default config to toml: {err}"))
            .unwrap();
        log::debug!("Checking if the working directory exists");
        if !settings.work_dir.exists() {
            log::debug!("Creating working directory");
            std::fs::create_dir_all(&settings.work_dir).unwrap();
        }
        let mut config_file = settings.work_dir;
        config_file.push(CARDANO_CONFIG_FILE_NAME);
        log::debug!("Checking if the config file exists");
        if !config_file.exists() {
            let path = config_file.to_str().expect("Failed to parse config file path buf to string");
            let mut f = File::create(&config_file)
                .map_err(|err| anyhow!("Failed to create config file in {path}: {err}"))
                .unwrap();
            writeln!(f, "{toml}")
                .map_err(|err| anyhow!("Failed write config file in {path}: {err}"))
                .unwrap();
        }
        log::debug!("Building configuration");
        Config::builder()
            .add_source(config::File::from(config_file))
            .build()
            .map_err(|err| anyhow!("Failed to build configuration: {err}"))
            .unwrap()
    });
}

pub fn read_settings() -> HashMap<String, String> {
    SETTINGS
        .read()
        .map_err(|err| anyhow!("Failed to read from settings: {err}"))
        .unwrap()
        .clone()
        .try_deserialize::<HashMap<String, String>>()
        .map_err(|err| anyhow!("Failed to serialize settings: {err}"))
        .unwrap()
}

pub fn show_settings() {
    let settings = read_settings();
    log::debug!("{settings:#?}");
}

pub fn read_setting(key: &str) -> Result<String> {
    log::debug!("Reading setting {key}");
    let settings = read_settings();
    let setting = settings
        .get(key)
        .ok_or_else(|| anyhow!("Failed to read setting {key}: invalid or does not exist"))
        .unwrap();
    Ok(setting.trim().to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::check_work_dir;

    #[test]
    fn test_settings() {
        let settings = Settings::default();
        log::debug!("{settings:#?}");
    }

    #[test]
    fn test_read_settings() {
        read_settings();
    }

    #[test]
    fn test_show_settings() {
        show_settings()
    }

    #[test]
    fn test_read_setting() -> Result<()> {
        let key = "work_dir";
        let value = read_setting(key)?;
        let work_dir = check_work_dir()?;
        let work_dir = work_dir.as_ref().to_str().unwrap();
        assert_eq!(value, work_dir);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Failed to read setting")]
    fn test_read_setting_fails() {
        std::panic::set_hook(Box::new(|_| {}));
        let key = "invalid_setting";
        read_setting(key).unwrap();
    }
}
