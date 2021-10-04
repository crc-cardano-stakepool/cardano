use crate::{async_command, get_component_path};
use anyhow::Result;

pub async fn fetch_tags(component: &str) -> Result<()> {
    let path = get_component_path(component).await?;
    let cmd = format!("cd {} && git fetch --all --recurse-submodules --tags", path);
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::fetch_tags;
    #[tokio::test]
    #[ignore]
    async fn test_fetch_tags() {
        unimplemented!();
    }
}
