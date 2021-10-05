use crate::{check_home_dir, print, set_env};
use anyhow::Result;
use std::collections::HashMap;

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
}
