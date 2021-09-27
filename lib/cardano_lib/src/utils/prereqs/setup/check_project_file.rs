use crate::print;
use anyhow::Result;

pub async fn check_project_file(component: &str) -> Result<()> {
    let msg = format!("Checking project file of {}", component);
    print("", &msg)?;
    Ok(())
}
