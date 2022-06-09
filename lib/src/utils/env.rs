use crate::{check_home_dir, print};
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
    print("", "Setting up environment")?;
    let home_dir = check_home_dir().await?;
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
    print("green", "Environment is ready")
}

#[cfg(test)]
mod test {
    // use super::*;

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
