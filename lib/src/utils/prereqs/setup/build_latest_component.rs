use crate::print;
use anyhow::Result;

pub async fn build_latest_component(component: &str) -> Result<()> {
    let msg = format!("Starting to build {}", component);
    print("", &msg)?;
    Ok(())
}
