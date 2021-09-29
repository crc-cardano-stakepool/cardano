use crate::{async_command, check_cabal, check_env, check_ghc, check_project_file, chownr, get_component_path, print};
use anyhow::Result;

pub async fn configure_build(component: &str, ghc_version: &str) -> Result<()> {
    print("", "Checking build dependencies")?;
    check_ghc().await?;
    check_cabal().await?;
    let path = get_component_path(component).await?;
    let cabal = check_env("CABAL_BIN")?;
    let ghc = check_env("GHC_BIN")?;
    print("", "Updating Cabal")?;
    let cmd = format!("cd {} && {} update", path, cabal);
    async_command(&cmd).await?;
    print("green", "Updated Cabal successfully")?;
    check_project_file(component).await?;
    let cmd = format!(
        "cd {} && {} configure --with-compiler={}-{}",
        path, cabal, ghc, ghc_version
    );
    async_command(&cmd).await?;
    chownr(&path).await?;
    let msg = format!("Configured build of {} successfully", component);
    print("green", &msg)?;
    let msg = format!("Building {}", component);
    print("", &msg)?;
    let cmd = format!("cd {} && {} build all", path, cabal);
    async_command(&cmd).await?;
    chownr(&path).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::configure_build;
    #[tokio::test]
    #[ignore]
    async fn test_configure_build() {
        unimplemented!();
    }
}
