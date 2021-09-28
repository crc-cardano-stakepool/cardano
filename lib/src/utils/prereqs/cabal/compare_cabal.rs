use crate::{async_command_pipe, get_cabal_version, install_cabal, print};
use anyhow::Result;

pub async fn compare_cabal(cabal_bin_path: &str) -> Result<()> {
    let version = get_cabal_version();
    let cmd = format!("{} -V | head -n1 |awk {}", cabal_bin_path, "'{print $3}'");
    let installed_cabal = async_command_pipe(&cmd).await?;
    let installed_cabal = installed_cabal.trim();
    if version == installed_cabal {
        print("green", "Cabal is installed")?;
    } else {
        print("red", "Wrong version of Cabal")?;
        install_cabal().await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::compare_cabal;
    #[tokio::test]
    #[ignore]
    async fn test_compare_cabal() {
        unimplemented!();
    }
}
