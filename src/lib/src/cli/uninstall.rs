use anyhow::Result;

pub async fn uninstall_component(component: &str) -> Result<()> {
    if component == "cardano-node" {
        log::info!("Uninstalling cardano-node");
        log::warn!("Not yet implemented");
    }
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
    async fn test_uninstall_component() {
        let result = uninstall_component("cardano-node").await.is_ok();
        assert!(result)
    }
}
