use crate::{file_exists, install_libsodium, print};
use anyhow::Result;

pub async fn check_libsodium() -> Result<()> {
    print("", "Checking libsodium")?;
    let libsodium = "/usr/local/lib/pkconfig/libsodium.pc";
    if !file_exists(libsodium) {
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
