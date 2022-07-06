use crate::{check_cabal, check_ghc, check_ghcup, check_libsodium, check_secp256k1, setup_packages, Component, ShellConfig};
use anyhow::Result;

pub async fn setup(component: Component) -> Result<()> {
    match component {
        Component::Node => setup_node().await,
        Component::Cli => setup_node().await,
        Component::Wallet => setup_wallet().await,
    }
}
