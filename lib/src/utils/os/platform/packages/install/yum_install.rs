use crate::{install_package, print, process_success};
use anyhow::Result;

pub async fn yum_install(package: &str) -> Result<()> {
    let cmd = format!("rpm -q {}", package);
    if process_success(&cmd).await? {
        let msg = format!("{} is installed", package);
        print("green", &msg)?;
    } else {
        install_package("yum", package).await?
    }
    Ok(())
}
