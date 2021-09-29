use crate::{async_command, check_env, check_repo, chownr, get_libsodium_url, print, source_shell};
use anyhow::Result;

pub async fn install_libsodium() -> Result<()> {
    let libsodium_path = check_env("LIBSODIUM_DIR")?;
    let url = get_libsodium_url();
    check_repo(url, &libsodium_path, "libsodium").await?;
    let msg = format!("Installing libsodium to {}", libsodium_path);
    print("", &msg)?;
    let cd = format!("cd {}", libsodium_path);
    let checkout = "git checkout 66f017f1";
    let autogen = "./autogen.sh";
    let configure = "./configure";
    let make = "make";
    let sudo = "sudo make install";
    let cmd = format!("{}\n{}\n{}\n{}\n{}\n{}", cd, checkout, autogen, configure, make, sudo);
    async_command(&cmd).await?;
    chownr(&libsodium_path).await?;
    source_shell().await?;
    print("", "Successfully installed libsodium")?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::install_libsodium;
    #[tokio::test]
    #[ignore]
    async fn test_install_libsodium() {
        unimplemented!();
    }
}
