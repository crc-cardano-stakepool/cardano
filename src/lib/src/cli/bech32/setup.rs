use crate::{check_cabal, check_ghc, check_ghcup, setup_packages, ShellConfig};
use anyhow::Result;

pub async fn setup_bech32() -> Result<()> {
    log::info!("Setting up the system with bech32 build dependencies");
    setup_packages().await?;
    ShellConfig::setup_shell().await?;
    check_bech32_dependencies().await
}

pub async fn check_bech32_dependencies() -> Result<()> {
    log::info!("Checking bech32 build dependencies");
    check_ghcup().await?;
    check_ghc().await?;
    check_cabal().await
}
