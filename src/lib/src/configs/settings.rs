use crate::check_env;
use anyhow::{anyhow, Result};
use config::Config;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf, sync::RwLock};

lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let config_home = match check_env("XDG_CONFIG_HOME") {
            Ok(home) => {
                let mut config = PathBuf::from(&home);
                config.push(".cardano");
                config
            }
            Err(_) => {
                let home = dirs::home_dir().unwrap();
                let mut config = PathBuf::from(&home);
                config.push(".config");
                config.push(".cardano");
                config
            }
        };
        let config_home_path = config_home.to_str().unwrap();
        if !config_home.exists() {
            std::fs::create_dir_all(&config_home).unwrap();
        }
        let mut config = PathBuf::from(&config_home);
        let cardano_config_file = "cardano.toml";
        config.push(cardano_config_file);
        let path = config.to_str().unwrap();
        if !config.exists() {
            let mut f = File::create(&config).map_err(|err| anyhow!("Failed to create config file in {path}: {err}")).unwrap();
            let config_home_path = config_home_path.trim();
            writeln!(f, "WORK_DIR = \"{config_home_path}\"").unwrap();
            writeln!(f, "LOG_FILE = \"{config_home_path}/logs\"").unwrap();
        }
        Config::builder().add_source(config::File::from(config)).build().unwrap()
    });
}

pub fn show_settings() {
    let settings = SETTINGS
        .read()
        .unwrap()
        .clone()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    println!("{settings:?}");
}

pub fn get_setting(key: &str) -> Result<String> {
    let settings = SETTINGS.read().unwrap().clone().try_deserialize::<HashMap<String, String>>()?;
    Ok(settings.get(key).unwrap().to_owned())
}

#[cfg(test)]
mod test {
    use crate::check_work_dir;

    use super::*;

    #[test]
    fn test_show_settings() {
        show_settings()
    }

    #[tokio::test]
    async fn test_get_setting() -> Result<()> {
        let key = "WORK_DIR";
        let value = get_setting(key)?;
        let work_dir = check_work_dir()?;
        let work_dir = work_dir.to_str().unwrap();
        assert_eq!(value, work_dir);
        Ok(())
    }
}
