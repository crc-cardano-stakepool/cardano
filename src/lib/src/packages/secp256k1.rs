use crate::{
    async_command, check_env, check_repo, drop_privileges, ShellConfig,
    SECP256K1_URL,
};
use anyhow::Result;
use std::path::Path;

pub async fn check_secp256k1() -> Result<()> {
    log::debug!("Checking secp256k1");
    let pc = Path::new("/usr/local/lib/pkgconfig/libsecp256k1.pc");
    let so = Path::new("/usr/local/lib/libsecp256k1.so");
    let so_0 = Path::new("/usr/local/lib/libsecp256k1.so.0");
    let so_0_0_0 = Path::new("/usr/local/lib/libsecp256k1.so.0.0.0");
    let la = Path::new("/usr/local/lib/libsecp256k1.la");
    let a = Path::new("/usr/local/lib/libsecp256k1.a");
    if !(pc.is_file()
        && so.is_file()
        && la.is_file()
        && so_0.is_file()
        && so_0_0_0.is_file()
        && a.is_file())
    {
        log::warn!("secp256k1 is not installed");
        install_secp256k1().await?;
    }
    Ok(())
}

pub async fn install_secp256k1() -> Result<()> {
    log::info!("Installing secp256k1");
    let secp256k1_path = check_env("SECP_256_K_1_DIR")?;
    check_repo(SECP256K1_URL, &secp256k1_path).await?;
    let checkout = "git checkout ac83be33";
    let configure =
        "./configure --enable-module-schnorrsig --enable-experimental";
    let cmd = format!("cd {secp256k1_path} && {checkout} && ./autogen.sh && {configure} && make");
    async_command(&cmd).await?;
    let cmd = format!("cd {secp256k1_path} && sudo make install");
    async_command(&cmd).await?;
    async_command("sudo ldconfig").await?;
    drop_privileges()?;
    ShellConfig::source_shell().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_install_secp256k1() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_secp256k1() {
        unimplemented!();
    }
}
