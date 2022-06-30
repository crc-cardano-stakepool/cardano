use crate::{install_packages, update, DEBIAN_PACKAGES, NON_DEBIAN_PACKAGES};
use anyhow::{anyhow, Result};

pub async fn install_distro_packages(distro: &str) -> Result<()> {
    match distro {
        "Ubuntu" | "Debian" | "Linux Mint" => {
            let package_manager = "apt";
            update(package_manager).await?;
            install_packages(package_manager, &DEBIAN_PACKAGES).await
        }
        "Fedora" | "Red Hat" | "CentOs" => {
            let package_manager = "yum";
            update(package_manager).await?;
            install_packages(package_manager, &NON_DEBIAN_PACKAGES).await
        }
        _ => Err(anyhow!("Unsupported distro: {distro}")),
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_install_distro_packages() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_distro() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_check_distro_result() {
        unimplemented!();
    }
}
