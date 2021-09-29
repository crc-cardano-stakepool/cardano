use crate::get_component_path;
use anyhow::Result;

pub async fn get_project_file(component: &str) -> Result<String> {
    let path = get_component_path(component).await?;
    let project_file = format!("{}/cabal.project.local", path);
    Ok(project_file)
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
