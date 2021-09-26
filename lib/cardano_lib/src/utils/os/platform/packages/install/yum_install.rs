use crate::{install_package, process_success};
use anyhow::Result;

pub async fn yum_install(package: &str) -> Result<()> {
    let cmd = format!("rpm -q {}", package);
    if !process_success(&cmd).await? {
        install_package("yum", package).await?
    } 
    Ok(())
}
