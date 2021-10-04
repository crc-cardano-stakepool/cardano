use crate::{async_command, chownr, print};
use anyhow::Result;

pub async fn update_project_file(component: &str, file_path: &str) -> Result<()> {
    let package = format!("echo \"package cardano-crypto-praos\" >> {}", file_path);
    let libsodium_flag = format!("echo \"  flags: -external-libsodium-vrf\" >> {}", file_path);
    async_command(&package).await?;
    async_command(&libsodium_flag).await?;
    let msg = format!("Updated project file of {}", component);
    chownr(file_path).await?;
    print("green", &msg)?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::update_project_file;
    #[tokio::test]
    #[ignore]
    async fn test_update_project_file() {
        unimplemented!();
    }
}
