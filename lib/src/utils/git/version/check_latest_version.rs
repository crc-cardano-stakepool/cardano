use crate::get_request;
use anyhow::Result;
use serde_json::json;

pub async fn check_latest_version(component: &str) -> Result<String> {
    let release_url = format!(
        "https://api.github.com/repos/input-output-hk/{}/releases/latest",
        component
    );
    let response = get_request(&release_url).await?;
    let latest_node_version: String = json!(response)["tag_name"].to_string().trim().replace("char", "");
    Ok(latest_node_version)
}

#[cfg(test)]
mod test {
    // use crate::check_latest_version;
    #[tokio::test]
    #[ignore]
    async fn test_check_latest_version() {
        unimplemented!();
    }
}
