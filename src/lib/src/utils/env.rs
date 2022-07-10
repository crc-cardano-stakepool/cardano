use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    env::{set_var, var},
};

pub struct Environment;
impl Environment {
    pub fn check_env(key: &str) -> Result<String> {
        log::debug!("Checking environment variable: {key}");
        match var(key) {
            Ok(val) => {
                log::trace!("{key}={val}");
                Ok(val)
            }
            Err(e) => {
                Err(anyhow!("Failed to check environment variable {key}: {e}"))
            }
        }
    }

    pub fn set_env(key: &str, value: &str) {
        log::debug!("Setting environment variable {key}={value}");
        set_var(key, value);
    }

    pub fn setup_env() -> Result<()> {
        log::debug!("Setting up environment variables");
        let home_dir = dirs::home_dir().expect("Failed to read $HOME");
        let home_dir =
            home_dir.to_str().expect("Failed to parse $HOME to string");
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
            Self::set_env(key, value);
        }
        Ok(())
    }

    pub fn check_user() -> Result<String> {
        log::debug!("Checking user");
        let user = Self::check_env("USER")?;
        let user = if user != "root" {
            user
        } else {
            Self::check_env("SUDO_USER")?
        };
        log::debug!("user: {user}");
        let user = user.trim().to_string();
        Ok(user.trim().to_string())
    }

    pub fn set_confirm(confirm: bool) {
        if confirm {
            return Self::set_env("CONFIRM", "true");
        }
        Self::set_env("CONFIRM", "false")
    }

    pub fn check_confirm() -> Result<bool> {
        let confirm = Self::check_env("CONFIRM")?;
        if confirm.eq("true") {
            return Ok(true);
        }
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_env() -> Result<()> {
        let user = Environment::check_user()?;
        let checked_env_user = Environment::check_env("USER")?;
        assert_eq!(user, checked_env_user);
        Ok(())
    }

    #[test]
    fn test_set_env() -> Result<()> {
        let key = "TEST";
        let value = "VALUE";
        Environment::set_env(key, value);
        let result = Environment::check_env(key)?;
        assert_eq!(result, value);
        Ok(())
    }

    #[test]
    fn test_setup_env() -> Result<()> {
        Environment::setup_env()?;
        let home_dir = dirs::home_dir().expect("Failed to read $HOME");
        let home_dir =
            home_dir.to_str().expect("Failed to parse $HOME to string");
        let ghcup_dir = format!("{home_dir}/.ghcup");
        let ghcup_bin = format!("{ghcup_dir}/bin/ghcup");
        let ghc_bin = format!("{ghcup_dir}/bin/ghc");
        let cabal_bin = format!("{ghcup_dir}/bin/cabal");
        let result = Environment::check_env("GHCUP_DIR")?;
        assert_eq!(result, ghcup_dir);
        let result = Environment::check_env("GHCUP_BIN")?;
        assert_eq!(result, ghcup_bin);
        let result = Environment::check_env("GHC_BIN")?;
        assert_eq!(result, ghc_bin);
        let result = Environment::check_env("CABAL_BIN")?;
        assert_eq!(result, cabal_bin);
        Ok(())
    }

    #[test]
    fn test_check_user() -> Result<()> {
        let user = Environment::check_user()?;
        log::debug!("user: {user}");
        let user_env = Environment::check_env("USER")?;
        log::debug!("user_env: {user_env}");
        assert_eq!(user, user_env);
        Ok(())
    }
}
