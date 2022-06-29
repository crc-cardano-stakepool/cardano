use anyhow::Result;
use log::info;

pub async fn uninstall_component(component: &str) -> Result<()> {
    if component == "cardano-node" {
        info!("Uninstalling cardano-node");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::setup;

    #[tokio::test]
    async fn test_logger() {
        setup();
        let result = uninstall_component("cardano-node").await.is_ok();
        assert!(result)
    }
}
