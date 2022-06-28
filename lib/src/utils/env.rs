use crate::check_home_dir;
use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    env::{set_var, var},
    path::PathBuf,
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
    let home_dir = check_home_dir().await?;
    let mut ghcup_dir = PathBuf::from(&home_dir);
    ghcup_dir.push(".ghcup");
    let mut ghcup_bin = PathBuf::from(&ghcup_dir);
    ghcup_bin.push("bin");
    ghcup_bin.push("ghcup");
    let mut ghc_bin = PathBuf::from(&ghcup_dir);
    ghc_bin.push("bin");
    ghc_bin.push("ghc");
    let mut cabal_bin = PathBuf::from(&ghcup_dir);
    cabal_bin.push("bin");
    cabal_bin.push("cabal");
    let map: HashMap<&str, PathBuf> = HashMap::from([
        ("GHCUP_DIR", ghcup_dir),
        ("GHCUP_BIN", ghcup_bin),
        ("GHC_BIN", ghc_bin),
        ("CABAL_BIN", cabal_bin),
    ]);
    for (key, value) in map {
        if let Some(value) = value.to_str() {
            set_env(key, value);
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_check_env() -> Result<()> {
        let user = crate::check_user().await?;
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
        let home_dir = check_home_dir().await?;
        let home_dir = home_dir.to_str().unwrap();
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
