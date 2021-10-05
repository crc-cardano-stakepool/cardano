use crate::{async_command, check_latest_version, chownr, fetch_tags, get_component_path, print};
use anyhow::Result;

pub async fn checkout_latest_release(component: &str) -> Result<()> {
    let msg = format!("Checking out latest {} release", component);
    let version = check_latest_version(component).await?;
    let path = get_component_path(component).await?;
    let cmd = format!("cd {} && git checkout tags/{}", path, version);
    print("", &msg)?;
    fetch_tags(component).await?;
    async_command(&cmd).await?;
    chownr(&path).await
}

#[cfg(test)]
mod test {
    // use crate::checkout_latest_release;
    #[tokio::test]
    #[ignore]
    async fn test_checkout_latest_release() {
        unimplemented!();
    }
}
