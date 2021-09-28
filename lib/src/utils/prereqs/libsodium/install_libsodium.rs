use crate::print;
use anyhow::Result;

pub async fn install_libsodium() -> Result<()> {
    print("", "Installing libsodium")?;
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
