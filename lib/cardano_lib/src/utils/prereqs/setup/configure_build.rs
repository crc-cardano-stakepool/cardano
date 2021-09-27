use crate::print;
use anyhow::Result;

pub async fn configure_build(component: &str) -> Result<()> {
    let msg = format!("Configuring the build of {}", component);
    print("", &msg)?;
    Ok(())
}
