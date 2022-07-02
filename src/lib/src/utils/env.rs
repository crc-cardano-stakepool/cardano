use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    env::{set_var, var},
};

pub fn check_env(key: &str) -> Result<String> {
    log::info!("Checking environment variable: {key}");
    match var(key) {
        Ok(val) => {
            log::debug!("{key}={val}");
            Ok(val)
        }
        Err(e) => Err(anyhow!("Failed to check environment variable {key}: {e}")),
    }
}

pub fn set_env(key: &str, value: &str) {
    log::info!("Setting {key}={value}");
    set_var(key, value);
}

pub fn setup_env() -> Result<()> {
    log::info!("Setting up environment variables");
    let home_dir = dirs::home_dir().expect("Failed to read $HOME");
    let home_dir = home_dir.to_str().expect("Failed to parse $HOME to string");
    let ghcup_dir = format!("{home_dir}/.ghcup");
    let ghcup_bin = format!("{ghcup_dir}/bin/ghcup");
    let ghc_bin = format!("{ghcup_dir}/bin/ghc");
    let cabal_bin = format!("{ghcup_dir}/bin/cabal");
    let map: HashMap<&str, &String> = HashMap::from([
        ("GHCUP_DIR", &ghcup_dir),
        ("GHCUP_BIN", &ghcup_bin),
        ("GHC_BIN", &ghc_bin),
        ("CABAL_BIN", &cabal_bin),
    ]);
    for (key, value) in map {
        set_env(key, value);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_check_env() -> Result<()> {
        let user = crate::check_user()?;
        let checked_env_user = check_env("USER")?;
        assert_eq!(user, checked_env_user);
        Ok(())
    }

    #[test]
    fn test_set_env() -> Result<()> {
        let key = "TEST";
        let value = "VALUE";
        set_env(key, value);
        let result = check_env(key)?;
        assert_eq!(result, value);
        Ok(())
    }

    #[test]
    fn test_setup_env() -> Result<()> {
        setup_env()?;
        let home_dir = dirs::home_dir().expect("Failed to read $HOME");
        let home_dir = home_dir.to_str().expect("Failed to parse $HOME to string");
        let ghcup_dir = format!("{home_dir}/.ghcup");
        let ghcup_bin = format!("{ghcup_dir}/bin/ghcup");
        let ghc_bin = format!("{ghcup_dir}/bin/ghc");
        let cabal_bin = format!("{ghcup_dir}/bin/cabal");
        let result = check_env("GHCUP_DIR")?;
        assert_eq!(result, ghcup_dir);
        let result = check_env("GHCUP_BIN")?;
        assert_eq!(result, ghcup_bin);
        let result = check_env("GHC_BIN")?;
        assert_eq!(result, ghc_bin);
        let result = check_env("CABAL_BIN")?;
        assert_eq!(result, cabal_bin);
        Ok(())
    }
}
