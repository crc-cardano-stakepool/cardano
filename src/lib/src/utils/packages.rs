use crate::{async_command, async_command_pipe, pipe, process_success, SystemInfo, DEBIAN_PACKAGES, NON_DEBIAN_PACKAGES};
use anyhow::{anyhow, Result};

pub async fn check_platform() -> Result<String> {
    let platform = async_command_pipe("uname").await;
    match platform {
        Ok(platform) => Ok(platform),
        Err(e) => Err(anyhow!("{e}")),
    }
}

pub async fn setup_packages() -> Result<()> {
    let output = check_platform().await?;
    let platform = output.as_str().trim();
    match platform {
        "linux" | "Linux" => {
            let distro = SystemInfo::get_sysinfo();
            install_distro_packages(&distro).await
        }
        "darwin" | "Darwin" => Err(anyhow!("macOS is currently unsupported")),
        _ => Err(anyhow!("Unsupported platform: {platform}")),
    }
}

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

pub async fn check_package(package_manager: &str, package: &str) -> Result<()> {
    match package_manager {
        "apt" => apt_install(package).await,
        "yum" => yum_install(package).await,
        _ => Err(anyhow!("Failed checking {package}")),
    }
}

pub async fn update(package_manager: &str) -> Result<()> {
    let cmd = format!("sudo {package_manager} update -y && sudo {package_manager} upgrade -y");
    async_command(&cmd).await?;
    Ok(())
}

pub async fn apt_install(package: &str) -> Result<()> {
    let cmd = format!("dpkg -s {}", package.trim());
    let piped_cmd = "grep installed";
    if let Ok(result) = pipe(&cmd, piped_cmd).await {
        if result.trim().is_empty() {
            install_package("apt", package).await
        } else {
            Ok(())
        }
    } else {
        Err(anyhow!("Failed installing {package}"))
    }
}

pub async fn install_package(package_manager: &str, package: &str) -> Result<()> {
    let cmd = format!("sudo {package_manager} install {package} -y");
    let process = async_command(&cmd).await;
    match process {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed installing {package} with error: {e}")),
    }
}

pub async fn install_packages(package_manager: &str, packages: &[&str]) -> Result<()> {
    for package in packages {
        check_package(package_manager, package).await?;
    }
    Ok(())
}

pub async fn yum_install(package: &str) -> Result<()> {
    let cmd = format!("rpm -q {package}");
    if !process_success(&cmd).await? {
        install_package("yum", package).await
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_check_platform() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_install_distro_packages() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_update() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_package() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_setup_packages() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_yum_install() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_install_packages() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_install_package() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_apt_install() {
        unimplemented!();
    }
}
