use crate::{check_env, file_exists, install_ghcup, is_dir, print};
use anyhow::Result;

pub async fn check_ghcup() -> Result<()> {
    print("", "Checking GHCup")?;
    let ghcup_dir = check_env("GHCUP_DIR")?;
    let ghcup_bin = check_env("GHCUP_BIN")?;
    if is_dir(&ghcup_dir) {
        if file_exists(&ghcup_bin) {
            print("green", "GHCup is installed")?;
        } else {
            print("red", "Failed installing GHCup")?;
        }
    } else {
        print("red", "GHCup is not installed")?;
        install_ghcup().await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::check_ghcup;
    #[tokio::test]
    #[ignore]
    async fn test_check_ghcup() {
        unimplemented!();
    }
}
