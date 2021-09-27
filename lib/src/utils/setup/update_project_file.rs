use crate::print;
use anyhow::Result;

pub async fn update_project_file(component: &str) -> Result<()> {
    let msg = format!("Updating project file of {}", component);
    print("", &msg)?;
    Ok(())
}