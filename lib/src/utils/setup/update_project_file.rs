use crate::print;
use anyhow::Result;

pub async fn update_project_file(component: &str) -> Result<()> {
    let msg = format!("Updating project file of {}", component);
    print("", &msg)?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::update_project_file;
    #[tokio::test]
    #[ignore]
    async fn test_update_project_file() {
        unimplemented!();
    }
}
