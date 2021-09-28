use crate::async_command_pipe;
use anyhow::Result;

pub async fn check_installed_version(component: &str) -> Result<String> {
    let cmd = format!("type {}", component);
    let installed = async_command_pipe(&cmd).await?;
    if !installed.contains("not found") {
        let cmd = format!("{} --version | awk {} | head -n1", component, "'{print $2}'");
        let version = async_command_pipe(&cmd).await?;
        let installed_version: String = String::from(version.trim());
        Ok(installed_version)
    } else {
        Ok(String::from("not found"))
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
