use crate::{
    async_command, async_user_command, check_env, check_repo, chownr, export_shell_variables, file_exists, print,
    LIBSODIUM_URL,
};
use anyhow::Result;

pub async fn install_secp256k1() -> Result<()> {
    let secp256k1_path = check_env("SECP256K1_DIR")?;
    let url = LIBSODIUM_URL;
    check_repo(url, &secp256k1_path, "secp256k1").await?;
    let checkout = "git checkout ac83be33";
    let autogen = "./autogen.sh";
    let configure = "./configure";
    let make = "make";
    let cd = format!("cd {secp256k1_path}\n{checkout}\n{autogen}\n{configure}\n{make}\n");
    let sudo = "sudo make install";
    let cmd = format!("cd {secp256k1_path}\n{sudo}");
    async_user_command(&cd).await?;
    async_command(&cmd).await?;
    chownr(&secp256k1_path).await?;
    export_shell_variables().await?;
    print("green", "Successfully installed secp256k1")
}

pub async fn check_secp256k1() -> Result<()> {
    print("", "Checking secp256k1")?;
    let pc = "/usr/local/lib/pkgconfig/libsecp256k1.pc";
    let so = "/usr/local/lib/libsecp256k1.so";
    let so_0 = "/usr/local/lib/libsecp256k1.so.0";
    let so_0_0_0 = "/usr/local/lib/libsecp256k1.so.0.0.0";
    let la = "/usr/local/lib/libsecp256k1.la";
    let a = "/usr/local/lib/libsecp256k1.a";
    if !(file_exists(pc)
        && file_exists(so)
        && file_exists(la)
        && file_exists(so_0)
        && file_exists(so_0_0_0)
        && file_exists(a))
    {
        install_secp256k1().await?;
    }
    print("green", "libsecp256k1 is installed")
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
