use crate::{async_command_pipe, get_ghc_version, install_ghc, print};
use anyhow::Result;

pub async fn compare_ghc(ghc_bin_path: &str) -> Result<()> {
    let version = get_ghc_version();
    let cmd = format!("{} -V | awk {}", ghc_bin_path, "'{print $8}'");
    let installed_ghc = async_command_pipe(&cmd).await?;
    let installed_ghc = installed_ghc.trim();
    if version == installed_ghc {
        print("green", "GHC is installed")?;
    } else {
        print("red", "Wrong version of GHC")?;
        install_ghc().await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::check_ghc;
    #[tokio::test]
    #[ignore]
    async fn test_check_ghc() {
        unimplemented!();
    }
}
