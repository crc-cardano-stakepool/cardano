use crate::{async_command_pipe, install_packages, update, DEBIAN_PACKAGES, NON_DEBIAN_PACKAGES};
use anyhow::{anyhow, Result};

pub fn check_distro_result(distro: Result<String>) -> Result<String> {
    match distro {
        Ok(result) => Ok(result),
        Err(e) => Err(anyhow!("Failed checking distribution with error: {e}")),
    }
}

pub async fn check_distro() -> Result<String> {
    let cmd = format!("cat /etc/*ease | grep ID_LIKE | awk -F '=' {}", "'{print $2}'");
    let distro = async_command_pipe(&cmd).await;
    match distro {
        Ok(distro) => {
            if distro.is_empty() {
                let cmd = format!("cat /etc/*ease | grep ID | awk -F '=' {} | tail -n1", "'{print $2}'");
                let distro = async_command_pipe(&cmd).await;
                check_distro_result(distro)
            } else {
                check_distro_result(Ok(distro))
            }
        }
        Err(e) => Err(anyhow!("Failed checking distro with error: {e}")),
    }
}

pub async fn install_distro_packages(distro: &str) -> Result<()> {
    match distro {
        "ubuntu" | "debian" => {
            let package_manager = "apt";
            update(package_manager).await?;
            install_packages(package_manager, &DEBIAN_PACKAGES).await
        }
        "Fedora" | "Hat" | "CentOs" => {
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
