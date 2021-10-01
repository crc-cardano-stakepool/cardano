use crate::{
    async_command, check_user, get_cabal_version, get_ghc_version,
    get_ghcup_install_url, print,
};
use anyhow::Result;

pub async fn install_ghcup() -> Result<()> {
    print("", "Installing GHCup")?;
    let user = check_user().await?;
    let ghc_version = get_ghc_version();
    let cabal_version = get_cabal_version();
    let ghcup_install_url = get_ghcup_install_url();
    let non_interactive = "export BOOTSTRAP_HASKELL_NONINTERACTIVE=1";
    let ghc = format!("export BOOTSTRAP_HASKELL_GHC_VERSION={}", ghc_version);
    let cabal =
        format!("export BOOTSTRAP_HASKELL_CABAL_VERSION={}", cabal_version);
    let call = format!(
        "$(curl --proto '=https' --tlsv1.2 -sSf {})",
        ghcup_install_url
    );
    let ghcup_script =
        format!("\n{}\n{}\n{}\n{}", non_interactive, ghc, cabal, call);
    let cmd = format!("su - {} -c \"eval {}\"", user, ghcup_script);
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::install_ghcup;
    #[tokio::test]
    #[ignore]
    async fn test_install_ghcup() {
        unimplemented!();
    }
}
