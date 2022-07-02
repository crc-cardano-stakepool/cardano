use crate::{async_command, async_user_command, check_env, check_repo, chownr, export_shell_variables, file_exists, SECP256K1_URL};
use anyhow::Result;

pub async fn check_secp256k1() -> Result<()> {
    let pc = "/usr/local/lib/pkgconfig/libsecp256k1.pc";
    let so = "/usr/local/lib/libsecp256k1.so";
    let so_0 = "/usr/local/lib/libsecp256k1.so.0";
    let so_0_0_0 = "/usr/local/lib/libsecp256k1.so.0.0.0";
    let la = "/usr/local/lib/libsecp256k1.la";
    let a = "/usr/local/lib/libsecp256k1.a";
    if !(file_exists(pc) && file_exists(so) && file_exists(la) && file_exists(so_0) && file_exists(so_0_0_0) && file_exists(a)) {
        install_secp256k1().await?;
    }
    Ok(())
}

pub async fn install_secp256k1() -> Result<()> {
    let secp256k1_path = check_env("SECP_256_K_1_DIR")?;
    check_repo(SECP256K1_URL, &secp256k1_path).await?;
    let checkout = "git checkout ac83be33";
    let autogen = "./autogen.sh";
    let configure = "./configure --enable-module-schnorrsig --enable-experimental";
    let make = "make";
    let cd = format!("cd {secp256k1_path}\n{checkout}\n{autogen}\n{configure}\n{make}\n");
    let sudo = "sudo make install";
    let cmd = format!("cd {secp256k1_path}\n{sudo}");
    async_user_command(&cd).await?;
    async_command(&cmd).await?;
    async_command("sudo ldconfig").await?;
    chownr(&secp256k1_path).await?;
    export_shell_variables().await?;
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
