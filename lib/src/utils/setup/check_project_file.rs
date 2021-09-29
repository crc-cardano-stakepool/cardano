use crate::{file_exists, get_component_path, print, process_success, update_project_file};
use anyhow::Result;

pub async fn check_project_file(component: &str) -> Result<()> {
    let msg = format!("Checking project file of {}", component);
    print("", &msg)?;
    let path = get_component_path(component).await?;
    let project_file = format!("{}/cabal.project.local", path);
    if !file_exists(&project_file) {
        update_project_file(component, &project_file).await?;
    } else {
        let package = format!("grep -q \"package cardano-crypto-praos\" {}", project_file);
        let libsodium_flag = format!("grep -q \"flags: -external-libsodium-vrf\" {}", project_file);
        let cmd = format!("{} && {}", package, libsodium_flag);
        if process_success(&cmd).await? {
            print("green", "Project file is configured to build with IOHK libsodium fork")?;
        } else {
            update_project_file(component, &project_file).await?;
        }
    }
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
