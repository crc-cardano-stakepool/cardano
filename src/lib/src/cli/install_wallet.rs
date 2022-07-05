use crate::{check_install, copy_binary, Component, ShellConfig};
use anyhow::Result;

pub async fn install_wallet() -> Result<()> {
    build_wallet().await?;
    copy_binary(Component::Wallet).await?;
    check_install(Component::Wallet).await?;
    ShellConfig::source_shell().await
}

pub async fn build_wallet() -> Result<()> {
    unimplemented!();
}
