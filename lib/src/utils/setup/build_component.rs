use crate::{check_project_file, configure_build, print};
use anyhow::Result;

pub async fn build_component(component: &str) -> Result<()> {
    let msg = format!("Building {}", component);
    print("", &msg)?;
    configure_build(component).await?;
    check_project_file(component).await?;
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
