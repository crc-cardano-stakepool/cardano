use anyhow::Result;

pub async fn uninstall_bech32() -> Result<()> {
    log::info!("Uninstalling bech32");
    log::warn!("Not yet implemented");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_uninstall_bech32() {
        let result = uninstall_bech32().await.is_ok();
        assert!(result);
    }
}
