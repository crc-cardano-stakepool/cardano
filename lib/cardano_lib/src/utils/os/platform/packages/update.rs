use crate::spinner_cmd;
use anyhow::Result;

pub async fn update(package_manager: &str) -> Result<()> {
    let cmd = format!(
        "sudo {} update -y && sudo {} upgrade -y",
        package_manager, package_manager
    );
    spinner_cmd(&cmd, "Updating", "Finished updating").await?;
    Ok(())
}
