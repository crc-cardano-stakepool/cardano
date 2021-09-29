use crate::{clone_component, configure_build, get_ghc_version, print};
use anyhow::Result;

pub async fn build_component(component: &str) -> Result<()> {
    let ghc_version = get_ghc_version();
    clone_component(component).await?;
    configure_build(component, ghc_version).await?;
    let msg = format!("Successfully built {}", component);
    print("", &msg)?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::configure_build;
    #[tokio::test]
    #[ignore]
    async fn test_configure_build() {
        unimplemented!();
    }
}
