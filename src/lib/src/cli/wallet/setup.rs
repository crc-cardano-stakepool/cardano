use crate::{check_cabal, check_ghc, check_ghcup, setup_packages, ShellConfig};
use anyhow::Result;

pub async fn setup_wallet() -> Result<()> {
    log::info!("Setting up the system with build dependencies");
    setup_packages().await?;
    ShellConfig::setup_shell().await?;
    check_wallet_dependencies().await
}

pub async fn check_wallet_dependencies() -> Result<()> {
    log::info!("Checking build dependencies");
    check_ghcup().await?;
    check_ghc().await?;
    check_cabal().await
}
