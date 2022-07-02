use crate::{
    async_command, async_user_command, check_env, check_repo, drop_privileges, export_shell_variables, file_exists, LIBSODIUM_URL,
};
use anyhow::Result;

pub async fn check_libsodium() -> Result<()> {
    log::info!("Checking if libdsodium is installed");
    let pc = "/usr/local/lib/pkgconfig/libsodium.pc";
    let so = "/usr/local/lib/libsodium.so";
    let so_23 = "/usr/local/lib/libsodium.so.23";
    let so_23_3_0 = "/usr/local/lib/libsodium.so.23.3.0";
    let la = "/usr/local/lib/libsodium.la";
    let a = "/usr/local/lib/libsodium.a";
    if !(file_exists(pc) && file_exists(so) && file_exists(la) && file_exists(so_23_3_0) && file_exists(so_23) && file_exists(a)) {
        log::warn!("Libsodium is not installed");
        install_libsodium().await?;
    }
    Ok(())
}

pub async fn install_libsodium() -> Result<()> {
    log::info!("Installing libsodium");
    let libsodium_path = check_env("LIBSODIUM_DIR")?;
    check_repo(LIBSODIUM_URL, &libsodium_path).await?;
    let checkout = "git checkout 66f017f1";
    let autogen = "./autogen.sh";
    let configure = "./configure";
    let make = "make";
    let cd = format!("cd {libsodium_path}\n{checkout}\n{autogen}\n{configure}\n{make}\n");
    let sudo = "sudo make install";
    let cmd = format!("cd {libsodium_path}\n{sudo}");
    async_user_command(&cd).await?;
    async_command(&cmd).await?;
    drop_privileges()?;
    export_shell_variables().await?;
    Ok(())
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
