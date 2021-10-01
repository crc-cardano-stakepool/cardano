use crate::{file_exists, install_libsodium, print};
use anyhow::Result;

pub async fn check_libsodium() -> Result<()> {
    print("", "Checking libsodium")?;
    let pc = "/usr/local/lib/pkgconfig/libsodium.pc";
    let so = "/usr/local/lib/libsodium.so";
    let la = "/usr/local/lib/libsodium.la";
    let a = "/usr/local/lib/libsodium.a";
    if !(file_exists(pc)
        && file_exists(so)
        && file_exists(la)
        && file_exists(a))
    {
        install_libsodium().await?;
    }
    print("green", "libsodium is installed")?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::check_libsodium;
    #[tokio::test]
    #[ignore]
    async fn test_check_libsodium() {
        unimplemented!();
    }
}
