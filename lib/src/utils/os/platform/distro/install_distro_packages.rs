use crate::{install_packages, update, PACKAGES};
use anyhow::Result;

pub async fn install_distro_packages(distro: &str) -> Result<()> {
    match distro {
        "ubuntu" | "debian" => {
            let package_manager = "apt";
            if let Some(packages) = PACKAGES.get("debian_packages") {
                update(package_manager).await?;
                install_packages(package_manager, packages).await?;
            }
        }
        "Fedora" | "Hat" | "CentOs" => {
            let package_manager = "yum";
            if let Some(packages) = PACKAGES.get("non_debian_packages") {
                install_packages(package_manager, packages).await?;
            }
        }
        _ => panic!("Unsupported distro: {}", distro),
    }
    Ok(())
}
