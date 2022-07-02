use anyhow::{anyhow, Result};
use config::Config;
use serde::Serialize;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf, sync::RwLock};

use crate::CARDANO_CONFIG_FILE_NAME;

#[derive(Hash, Serialize, Debug, Eq, PartialEq)]
pub struct Settings {
    work_dir: PathBuf,
    log_file: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        log::debug!("Creating default settings");
        let mut work_dir = dirs::config_dir().expect("Failed to read XDG_CONFIG_HOME");
        work_dir.push(".cardano");
        let mut log_file = PathBuf::from(&work_dir);
        log_file.push("logs");
        Self { work_dir, log_file }
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
            writeln!(f, "{}", toml)
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
    log::info!("Reading settings");
    SETTINGS
        .read()
        .unwrap()
        .clone()
        .try_deserialize::<HashMap<String, String>>()
        .map_err(|err| anyhow!("Failed to serialize settings: {err}"))
        .unwrap()
}

pub fn show_settings() {
    log::info!("Showing settings");
    let settings = read_settings();
    log::debug!("{settings:?}");
}

pub fn read_setting(key: &str) -> Result<String> {
    log::info!("settings");
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
    fn test_get_setting() -> Result<()> {
        let key = "work_dir";
        let value = read_setting(key)?;
        let work_dir = check_work_dir()?;
        let work_dir = work_dir.to_str().unwrap();
        assert_eq!(value, work_dir);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Failed to read setting")]
    fn test_get_setting_fails() {
        std::panic::set_hook(Box::new(|_| {}));
        let key = "invalid_setting";
        read_setting(key).unwrap();
    }
}
