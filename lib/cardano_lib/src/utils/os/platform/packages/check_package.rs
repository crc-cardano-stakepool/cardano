use crate::{apt_install, print, yum_install};
use anyhow::Result;

pub async fn check_package(package_manager: &str, package: &str) -> Result<()> {
    println!("Checking for {}", package);
    match package_manager {
        "apt" => apt_install(package).await?,
        "yum" => yum_install(package).await?,
        _ => {
            let msg = format!("Failed checking {}", package);
            print("red", &msg)?
        }
    };
    Ok(())
}
