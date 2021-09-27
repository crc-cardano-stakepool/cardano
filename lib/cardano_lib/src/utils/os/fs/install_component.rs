use crate::print;
use anyhow::Result;

pub async fn install_component(component: &str) -> Result<()> {
    let msg = format!("Installing {}", component);
    print("", &msg)?;
    Ok(())
}
