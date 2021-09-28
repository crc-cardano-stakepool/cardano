use crate::{print, update_project_file};
use anyhow::Result;

pub async fn check_project_file(component: &str) -> Result<()> {
    let msg = format!("Checking project file of {}", component);
    print("", &msg)?;
    update_project_file(component).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::check_project_file;
    #[tokio::test]
    #[ignore]
    async fn test_check_project_file() {
        unimplemented!();
    }
}
