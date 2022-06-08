use crate::{install_packages, update, PACKAGES};
use anyhow::{anyhow, Result};

pub async fn install_distro_packages(distro: &str) -> Result<()> {
    match distro {
        "ubuntu" | "debian" => {
            let package_manager = "apt";
            if let Some(packages) = PACKAGES.get("debian_packages") {
                update(package_manager).await?;
                install_packages(package_manager, packages).await
            } else {
                Err(anyhow!("Failed checking packages"))
            }
        }
        "Fedora" | "Hat" | "CentOs" => {
            let package_manager = "yum";
            if let Some(packages) = PACKAGES.get("non_debian_packages") {
                update(package_manager).await?;
                install_packages(package_manager, packages).await
            } else {
                Err(anyhow!("Failed checking packages"))
            }
        }
        _ => Err(anyhow!("Unsupported distro: {}", distro)),
    }
}

#[cfg(test)]
mod test {
    // use crate::install_distro_packages;
    #[tokio::test]
    #[ignore]
    async fn test_install_distro_packages() {
        unimplemented!();
    }
}