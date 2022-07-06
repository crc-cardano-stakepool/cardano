use anyhow::Result;

pub async fn uninstall_wallet() -> Result<()> {
    log::info!("Uninstalling cardano-wallet");
    log::warn!("Not yet implemented");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_uninstall_wallet() {
        let result = uninstall_wallet().await.is_ok();
        assert!(result);
    }
}
