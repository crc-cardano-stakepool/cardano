use crate::{
    apt_install, async_command, async_command_pipe, check_distro, install_distro_packages, print, yum_install,
};
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
            print("green", "Detected linux")?;
            let output = check_distro().await?;
            let distro = output.as_str().trim();
            install_distro_packages(distro).await
        }
        "darwin" | "Darwin" => {
            print("red", "Detected macOS")?;
            Err(anyhow!("macOS is currently unsupported"))
        }
        _ => Err(anyhow!("Unsupported platform: {platform}")),
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
}
