use crate::{file_exists, install_libsodium, print};
use anyhow::Result;

pub async fn check_libsodium() -> Result<()> {
    print("", "Checking libsodium")?;
    let lib_files = &[
        "/usr/local/lib/libsodium.la",
        "/usr/local/lib/libsodium.a",
        "/usr/local/lib/libsodium.so",
    ];
    for file in lib_files {
        if !file_exists(file) {
            install_libsodium().await?;
        }
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
