use crate::check_home_dir;
use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    env::{set_var, var},
};

pub fn check_env(key: &str) -> Result<String> {
    match var(key) {
        Ok(val) => Ok(val),
        Err(e) => Err(anyhow!("couldn't interpret {key}: {e}")),
    }
}

pub fn set_env(key: &str, value: &str) {
    set_var(key, value);
}

pub async fn setup_env() -> Result<()> {
    let home_dir = check_home_dir()?;
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

    #[tokio::test]
    async fn test_setup_env() -> Result<()> {
        setup_env().await?;
        let home_dir = check_home_dir()?;
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
