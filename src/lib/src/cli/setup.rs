use crate::{check_cabal, check_ghc, check_ghcup, check_libsodium, check_secp256k1, setup_packages, Component, ShellConfig};
use anyhow::Result;

pub async fn setup(component: Component) -> Result<()> {
    match component {
        Component::Node => setup_node().await,
        Component::Cli => setup_node().await,
        Component::Wallet => setup_wallet().await,
    }
}

pub async fn setup_node() -> Result<()> {
    log::info!("Setting up the system with build dependencies");
    setup_packages().await?;
    ShellConfig::setup_shell().await?;
    check_node_dependencies().await?;
    Ok(())
}

pub async fn check_node_dependencies() -> Result<()> {
    log::info!("Checking build dependencies");
    check_ghcup().await?;
    check_ghc().await?;
    check_cabal().await?;
    check_libsodium().await?;
    check_secp256k1().await
}

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
