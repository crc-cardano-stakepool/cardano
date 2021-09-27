use crate::{check_dependencies, install_build_dependencies, print};
use anyhow::Result;

pub async fn prepare_build() -> Result<()> {
    print("", "Preparing build")?;
    install_build_dependencies().await?;
    check_dependencies().await?;
    Ok(())
}
