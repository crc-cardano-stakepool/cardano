use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::Value;

pub async fn get_request(url: &str) -> Result<Value> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Web 3")
        .send()
        .await?
        .json()
        .await;
    match response {
        Ok(result) => Ok(result),
        Err(e) => Err(anyhow!("Request failed with error: {}", e)),
    }
}

#[cfg(test)]
mod test {
    // use crate::get_request;
    #[tokio::test]
    #[ignore]
    async fn test_get_request() {
        unimplemented!();
    }
}
