use crate::{async_command, get_bin_path};
use anyhow::Result;

pub async fn check_installed_version(component: &str) -> Result<String> {
    let component_bin_path = get_bin_path(component).await?;
    let cmd = format!(
        "{} --version | awk {} | head -n1",
        component_bin_path, "'{print $2}'"
    );
    let version = async_command(&cmd).await?;
    let installed_version: String = String::from(version.trim());
    Ok(installed_version)
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
