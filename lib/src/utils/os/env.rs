use crate::{check_home_dir, print};
use std::{collections::HashMap, env::{var, set_var}};
use anyhow::{anyhow, Result};

pub fn check_env(key: &str) -> Result<String> {
    match var(key) {
        Ok(val) => Ok(val),
        Err(e) => Err(anyhow!("couldn't interpret {}: {}", key, e)),
    }
}

pub fn set_env(key: &str, value: &str) {
    set_var(key, value);
}

pub async fn setup_env() -> Result<()> {
    print("", "Setting up environment")?;
    let home_dir = check_home_dir().await?;
    let ghcup_dir = format!("{}/.ghcup", home_dir);
    let ghcup_bin = format!("{}/bin/ghcup", ghcup_dir);
    let ghc_bin = format!("{}/bin/ghc", ghcup_dir);
    let cabal_bin = format!("{}/bin/cabal", ghcup_dir);
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("GHCUP_DIR", &ghcup_dir);
    map.insert("GHCUP_BIN", &ghcup_bin);
    map.insert("GHC_BIN", &ghc_bin);
    map.insert("CABAL_BIN", &cabal_bin);
    for (key, value) in map {
        set_env(key, value);
    }
    print("green", "Environment is ready")
}

#[cfg(test)]
mod test {
    // use crate::setup_env;
    #[tokio::test]
    #[ignore]
    async fn test_setup_env() {
        unimplemented!();
    }
    #[test]
    #[ignore]
    fn test_set_env() {
        unimplemented!();
    }
    #[test]
    #[ignore]
    fn test_check_env() {
        unimplemented!();
    }
}
