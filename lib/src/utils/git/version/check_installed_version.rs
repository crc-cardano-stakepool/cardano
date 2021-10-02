use crate::{async_command_pipe, check_home_dir, file_exists};
use anyhow::Result;

pub async fn check_installed_version(component: &str) -> Result<String> {
    let home_dir = check_home_dir().await?;
    let install_dir = format!("{}/.local/bin", home_dir);
    let component_bin_path = format!("{}/{}", install_dir, component);
    if file_exists(&component_bin_path) {
        let cmd = format!(
            "{} --version | awk {} | head -n1",
            component_bin_path, "'{print $2}'"
        );
        let version = async_command_pipe(&cmd).await?;
        let installed_version: String = String::from(version.trim());
        Ok(installed_version)
    } else {
        Ok(String::from("Not installed"))
    }
}

#[cfg(test)]
mod test {
    // use crate::check_installed_version;
    #[tokio::test]
    #[ignore]
    async fn test_check_installed_version() {
        unimplemented!();
    }
}
