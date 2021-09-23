use super::color::print;
use super::process::async_command_pipe;
use anyhow::{anyhow, Result};
use console::Emoji;
use reqwest::Client;
use serde_json::{json, Value};

pub async fn check_version(component: &str) -> Result<bool> {
    let latest_version = check_installed_version(component).await?;
    let installed_version = check_latest_version(component).await?;
    if compare_version(&installed_version, &latest_version).await? {
        Ok(true)
    } else {
        let msg = format!("{} is not installed", component);
        print("red", &msg, Emoji("😔", ""))?;
        Ok(false)
    }
}

async fn check_installed_version(component: &str) -> Result<String> {
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

async fn get_request(url: &str) -> Result<Value> {
    let client = Client::new();
    let response = client.get(url).header("User-Agent", "Web 3").send().await?.json().await;
    match response {
        Ok(result) => Ok(result),
        Err(e) => Err(anyhow!("Request failed with error: {}", e)),
    }
}

async fn check_latest_version(component: &str) -> Result<String> {
    let release_url = format!(
        "https://api.github.com/repos/input-output-hk/{}/releases/latest",
        component
    );
    let response = get_request(&release_url).await?;
    let latest_node_version: String = json!(response)["tag_name"].to_string().trim().replace("\"", "");
    Ok(latest_node_version)
}

async fn compare_version(installed_version: &str, latest_version: &str) -> Result<bool> {
    if installed_version.eq(latest_version) {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn test_check_node_version() {
        unimplemented!();
    }
    #[test]
    #[ignore]
    fn test_fetch_latest_node_version() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_fetch_installed_node_version() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_compare_latest_node_version() {
        unimplemented!();
    }
}
