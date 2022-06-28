use crate::{
    async_command, async_user_command, check_env, check_repo, chownr, export_shell_variables, print, LIBSODIUM_URL,
};
use anyhow::Result;
use std::path::Path;

pub async fn check_libsodium() -> Result<()> {
    print("", "Checking libsodium")?;
    let pc = Path::new("/usr/local/lib/pkgconfig/libsodium.pc");
    let so = Path::new("/usr/local/lib/libsodium.so");
    let so_23 = Path::new("/usr/local/lib/libsodium.so.23");
    let so_23_3_0 = Path::new("/usr/local/lib/libsodium.so.23.3.0");
    let la = Path::new("/usr/local/lib/libsodium.la");
    let a = Path::new("/usr/local/lib/libsodium.a");
    if !(pc.exists() && so.exists() && la.exists() && so_23_3_0.exists() && so_23.exists() && a.exists()) {
        install_libsodium().await?;
    }
    print("green", "libsodium is installed")
}

pub async fn install_libsodium() -> Result<()> {
    let libsodium_path = check_env("LIBSODIUM_DIR")?;
    let path = Path::new(&libsodium_path);
    check_repo(LIBSODIUM_URL, path, "libsodium").await?;
    let checkout = "git checkout 66f017f1";
    let autogen = "./autogen.sh";
    let configure = "./configure";
    let make = "make";
    let cd = format!("cd {libsodium_path}\n{checkout}\n{autogen}\n{configure}\n{make}\n");
    let sudo = "sudo make install";
    let cmd = format!("cd {libsodium_path}\n{sudo}");
    async_user_command(&cd).await?;
    async_command(&cmd).await?;
    chownr(path)?;
    export_shell_variables().await?;
    print("green", "Successfully installed libsodium")
}

#[cfg(test)]
mod test {
    // use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_install_libsodium() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_get_ghcup_install_url() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_libsodium() {
        unimplemented!();
    }
}
