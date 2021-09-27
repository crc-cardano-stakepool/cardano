use crate::{check_dependencies, install_build_dependencies};
use anyhow::Result;

pub async fn prepare_build() -> Result<()> {
    println!("Preparing build");
    install_build_dependencies().await?;
    check_dependencies().await?;
    Ok(())
}
