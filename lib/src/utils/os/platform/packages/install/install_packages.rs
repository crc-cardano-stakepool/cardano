use crate::{async_command, check_package, print};
use anyhow::Result;

pub async fn install_packages(package_manager: &str, packages: &[&str]) -> Result<()> {
    println!("Updating");
    let cmd = format!(
        "sudo {} update -y && sudo {} upgrade -y",
        package_manager, package_manager
    );
    async_command(&cmd).await?;
    print("green", "Finished updating")?;
    for package in packages {
        check_package(package_manager, package).await?;
    }
    print("green", "Successfully installed packages")?;
    Ok(())
}
