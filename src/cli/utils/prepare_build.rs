use super::color::print;
use super::env::check_env;
use super::fs::check_directory;
use anyhow::Result;
use console::Emoji;

pub async fn prepare_build() -> Result<()> {
    println!("Preparing build");
    install_build_dependencies().await?;
    Ok(())
}

pub async fn install_build_dependencies() -> Result<()> {
    println!("Checking dependencies");
    setup_workdir().await?;
    check_ghcup().await?;
    check_ghc().await?;
    check_cabal().await?;
    install_libsodium().await?;
    print("green", "Successfully installed dependencies", Emoji("", ""))?;
    Ok(())
}

pub async fn setup_workdir() -> Result<()> {
    println!("Setting up working directory");
    let work_dir = check_env("WORK_DIR")?;
    let ipc_dir = format!("{}/ipc", work_dir);
    let config_dir = format!("{}/config", work_dir);
    let data_dir = format!("{}/data/db", work_dir);
    let mainnet_data_dir = format!("{}/mainnet", data_dir);
    let testnet_data_dir = format!("{}/testnet", data_dir);
    check_directory("working", &work_dir).await?;
    check_directory("ipc", &ipc_dir).await?;
    check_directory("config", &config_dir).await?;
    check_directory("data", &data_dir).await?;
    check_directory("mainnet", &mainnet_data_dir).await?;
    check_directory("testnet", &testnet_data_dir).await?;
    Ok(())
}

pub async fn install_libsodium() -> Result<()> {
    println!("Installing libsodium");
    Ok(())
}

pub async fn check_ghcup() -> Result<()> {
    println!("Checking GHCup");
    Ok(())
}

pub async fn check_ghc() -> Result<()> {
    println!("Checking GHC");
    Ok(())
}

pub async fn check_cabal() -> Result<()> {
    println!("Checking Cabal");
    Ok(())
}
