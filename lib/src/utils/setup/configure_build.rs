use crate::{async_user_command, check_env, print};
use anyhow::Result;

pub async fn configure_build(component: &str, ghc_version: &str, path: &str, cabal: &str) -> Result<()> {
    print("", "Configuring build")?;
    let ghc = check_env("GHC_BIN")?;
    let cmd = format!(
        "cd {} && {} configure --with-compiler={}-{}",
        path, cabal, ghc, ghc_version
    );
    async_user_command(&cmd).await?;
    let msg = format!("Configured build of {} successfully", component);
    print("green", &msg)
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
