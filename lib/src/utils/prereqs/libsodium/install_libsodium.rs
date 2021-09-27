use crate::print;
use anyhow::Result;

pub async fn install_libsodium() -> Result<()> {
    print("", "Installing libsodium")?;
    Ok(())
}
