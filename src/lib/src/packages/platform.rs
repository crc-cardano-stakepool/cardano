use crate::{
    async_command, async_command_pipe, drop_privileges, pipe, process_success,
    SystemInfo, DEBIAN_PACKAGES, NON_DEBIAN_PACKAGES,
};
use anyhow::{anyhow, Result};

pub async fn check_platform() -> Result<String> {
    log::debug!("Checking current platform");
    let platform = async_command_pipe("uname").await;
    match platform {
        Ok(platform) => Ok(platform),
        Err(e) => Err(anyhow!("{e}")),
    }
}

pub async fn setup_packages() -> Result<()> {
    log::debug!("Setting up required packages");
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
    log::debug!("Checking {distro} packages to install a cardano node");
    match distro {
        "Ubuntu" | "Debian" | "Linux Mint" => {
            let package_manager = "apt";
            update(package_manager).await?;
            check_packages(package_manager, &DEBIAN_PACKAGES).await
        }
        "Fedora" | "Red Hat" | "CentOs" => {
            let package_manager = "yum";
            update(package_manager).await?;
            check_packages(package_manager, &NON_DEBIAN_PACKAGES).await
        }
        _ => Err(anyhow!("Unsupported distro: {distro}")),
    }
}

pub async fn check_package(package_manager: &str, package: &str) -> Result<()> {
    log::debug!("Checking if {package} is installed");
    match package_manager {
        "apt" => apt_install(package).await,
        "yum" => yum_install(package).await,
        _ => Err(anyhow!("Failed checking {package}")),
    }
}

pub async fn update(package_manager: &str) -> Result<()> {
    log::info!("Updating system with {package_manager}");
    let cmd = format!("sudo {package_manager} update -y");
    async_command(&cmd).await?;
    Ok(())
}

pub async fn apt_install(package: &str) -> Result<()> {
    let cmd = format!("dpkg -s {}", package.trim());
    let piped_cmd = "grep installed";
    if let Ok(result) = pipe(&cmd, piped_cmd).await {
        if result.trim().is_empty() {
            return install_package("apt", package).await;
        }
        log::debug!("{package} is installed");
        return Ok(());
    }
    Err(anyhow!("Failed installing {package}"))
}

pub async fn install_package(
    package_manager: &str,
    package: &str,
) -> Result<()> {
    log::info!("Installing {package} with {package_manager}");
    let cmd = format!("sudo {package_manager} install {package} -y");
    let process = async_command(&cmd).await;
    match process {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed installing {package} with error: {e}")),
    }
}

pub async fn check_packages(
    package_manager: &str,
    packages: &[&str],
) -> Result<()> {
    log::debug!("Checking packages with {package_manager}");
    for package in packages {
        check_package(package_manager, package).await?;
    }
    drop_privileges()
}

pub async fn yum_install(package: &str) -> Result<()> {
    let cmd = format!("rpm -q {package}");
    if !process_success(&cmd).await? {
        return install_package("yum", package).await;
    }
    log::debug!("{package} is installed");
    Ok(())
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
