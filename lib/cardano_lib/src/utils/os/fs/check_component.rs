use crate::print;
use anyhow::Result;

pub async fn check_component(component: &str) -> Result<()> {
    let msg = format!("Checking {}", component);
    print("", &msg)?;
    Ok(())
}
