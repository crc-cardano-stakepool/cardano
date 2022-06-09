use crate::{
    async_command, async_user_command, check_env, check_repo, chownr, export_shell_variables, file_exists, print,
    LIBSODIUM_URL,
};
use anyhow::Result;

pub async fn install_libsodium() -> Result<()> {
    let libsodium_path = check_env("LIBSODIUM_DIR")?;
    let url = LIBSODIUM_URL;
    check_repo(url, &libsodium_path, "libsodium").await?;
    let checkout = "git checkout 66f017f1";
    let cd = format!("cd {}\n{}", libsodium_path, checkout);
    let autogen = "./autogen.sh";
    let configure = "./configure";
    let make = "make";
    let sudo = "sudo make install";
    let cmd = format!("{}\n{}\n{}\n{}\n{}", cd, autogen, configure, make, sudo);
    async_user_command(&cd).await?;
    async_command(&cmd).await?;
    chownr(&libsodium_path).await?;
    export_shell_variables().await?;
    print("green", "Successfully installed libsodium")
}

pub async fn check_libsodium() -> Result<()> {
    print("", "Checking libsodium")?;
    let pc = "/usr/local/lib/pkgconfig/libsodium.pc";
    let so = "/usr/local/lib/libsodium.so";
    let la = "/usr/local/lib/libsodium.la";
    let a = "/usr/local/lib/libsodium.a";
    if !(file_exists(pc) && file_exists(so) && file_exists(la) && file_exists(a)) {
        install_libsodium().await?;
    }
    print("green", "libsodium is installed")
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
