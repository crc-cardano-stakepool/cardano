use anyhow::Result;

pub async fn uninstall_node() -> Result<()> {
    log::info!("Uninstalling cardano-node");
    log::warn!("Not yet implemented");
    Ok(())
}

pub async fn uninstall_cli() -> Result<()> {
    log::info!("Uninstalling cardano-cli");
    log::warn!("Not yet implemented");
    Ok(())
}

pub async fn uninstall_wallet() -> Result<()> {
    log::info!("Uninstalling cardano-wallet");
    log::warn!("Not yet implemented");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_uninstall_node() {
        let result = uninstall_node().await.is_ok();
        assert!(result);
    }

    #[tokio::test]
    async fn test_uninstall_cli() {
        let result = uninstall_cli().await.is_ok();
        assert!(result);
    }
    #[tokio::test]
    async fn test_uninstall_wallet() {
        let result = uninstall_wallet().await.is_ok();
        assert!(result);
    }
}
