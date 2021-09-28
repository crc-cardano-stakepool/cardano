use anyhow::Result;

pub async fn install_mac_packages() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::install_mac_packages;
    #[tokio::test]
    #[ignore]
    async fn test_install_mac_packages() {
        unimplemented!();
    }
}
