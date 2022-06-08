use crate::async_command_pipe;
use anyhow::Result;

pub async fn check_latest_version(component: &str) -> Result<String> {
    let release_url = format!(
        "https://api.github.com/repos/input-output-hk/{}/releases/latest",
        component
    );
    let cmd = format!("curl -s {} | jq -r .tag_name", release_url);
    let response = async_command_pipe(&cmd).await?;
    Ok(String::from(response.trim()))
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    #[ignore]
    async fn test_check_latest_version() -> Result<()> {
        let version = check_latest_version("cardano-node").await?;
        assert_eq!(version, "1.34.1");
        Ok(())
    }
}
