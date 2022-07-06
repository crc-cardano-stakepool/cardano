use anyhow::Result;

pub async fn uninstall_address() -> Result<()> {
    log::info!("Uninstalling cardano-address");
    log::warn!("Not yet implemented");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_uninstall_address() {
        let result = uninstall_address().await.is_ok();
        assert!(result);
    }
}
