use crate::{check_cabal, check_ghc, check_ghcup, setup_packages, ShellConfig};
use anyhow::Result;

pub async fn setup_address() -> Result<()> {
    log::info!("Setting up the system with build dependencies");
    setup_packages().await?;
    ShellConfig::setup_shell().await?;
    check_address_dependencies().await
}

pub async fn check_address_dependencies() -> Result<()> {
    log::info!("Checking cardano-address build dependencies");
    check_ghcup().await?;
    check_ghc().await?;
    check_cabal().await
}
