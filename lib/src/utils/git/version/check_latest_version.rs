use crate::get_request;
use anyhow::Result;
use serde_json::json;

pub async fn check_latest_version(component: &str) -> Result<String> {
    let release_url = format!(
        "https://api.github.com/repos/input-output-hk/{}/releases/latest",
        component
    );
    let response = get_request(&release_url).await?;
    let latest_node_version: String = json!(response)["tag_name"].to_string().trim().replace("\"", "");
    Ok(latest_node_version)
}
