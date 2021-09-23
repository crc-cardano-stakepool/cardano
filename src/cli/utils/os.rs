use super::config_map::PACKAGES;
use super::process::{async_command, async_command_pipe, pipe};
use crate::cli::utils::color::print;
use anyhow::{anyhow, Result};
use console::Emoji;

pub async fn check_distro() -> Result<String> {
    println!("Checking distro");
    let cmd = format!("cat /etc/*ease | grep ID_LIKE | awk -F '=' {}", "'{print $2}'");
    let distro = async_command_pipe(&cmd).await;
    match distro {
        Ok(distro) => {
            if distro.is_empty() {
                let cmd = format!("cat /etc/*ease | grep ID | awk -F '=' {}", "'{print $2}'");
                let distro = async_command_pipe(&cmd).await;
                check_distro_result(distro)
            } else {
                Ok(distro)
            }
        }
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub fn check_distro_result(distro: Result<String>) -> Result<String> {
    match distro {
        Ok(result) => {
            let msg = format!("Detected {}", result.trim());
            print("green", &msg, Emoji("", ""))?;
            Ok(result)
        }
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub async fn check_platform() -> Result<String> {
    println!("Checking platform");
    let platform = async_command_pipe("uname").await;
    match platform {
        Ok(platform) => Ok(platform),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub async fn setup_packages() -> Result<()> {
    let output = check_platform().await?;
    let platform = output.as_str().trim();
    match platform {
        "linux" | "Linux" => {
            print("green", "Detected Linux", Emoji("", ""))?;
            let output = check_distro().await?;
            let distro = output.as_str().trim();
            install_distro_packages(distro).await?;
        }
        "darwin" | "Darwin" => {
            print("green", "Detected macOS", Emoji("", ""))?;
            install_mac_packages().await?
        }
        _ => panic!("Unsupported platform: {}", platform),
    }
    Ok(())
}

pub async fn install_distro_packages(distro: &str) -> Result<()> {
    println!("Installing {} dependencies", distro);
    match distro {
        "ubuntu" | "debian" => {
            let package_manager = "apt";
            if let Some(packages) = PACKAGES.get("debian_packages") {
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

pub async fn install_packages(package_manager: &str, packages: &[&str]) -> Result<()> {
    println!("Updating");
    let cmd = format!(
        "sudo {} update -y && sudo {} upgrade -y",
        package_manager, package_manager
    );
    async_command(&cmd).await?;
    print("green", "Finished updating", Emoji("", ""))?;
    for package in packages {
        check_package(package_manager, package).await?;
    }
    print("green", "Successfully installed packages", Emoji("", ""))?;
    Ok(())
}

pub async fn check_package(package_manager: &str, package: &str) -> Result<()> {
    println!("Checking for {}", package);
    match package_manager {
        "apt" => {
            let cmd = format!("dpkg -s {}", package.trim());
            let piped_cmd = "grep installed";
            let output = pipe(&cmd, piped_cmd).await;
            match output {
                Ok(result) => {
                    if result.trim().is_empty() {
                        install_package(package_manager, package).await?;
                    } else {
                        let msg = format!("{} is installed", package);
                        print("green", &msg, Emoji("", ""))?;
                    }
                }
                Err(_) => {
                    let msg = format!("Failed checking {}", package);
                    print("red", &msg, Emoji("", ""))?
                }
            }
        }
        "yum" => {
            let cmd = format!("rpm -q {}", package);
            let output = async_command_pipe(&cmd).await;
            match output {
                Ok(_) => {
                    let msg = format!("{} is installed", package);
                    print("green", &msg, Emoji("", ""))?;
                }
                Err(_) => install_package(package_manager, package).await?,
            }
        }
        _ => {
            let msg = format!("Failed checking {}", package);
            print("red", &msg, Emoji("", ""))?
        }
    };
    Ok(())
}

pub async fn install_package(package_manager: &str, package: &str) -> Result<()> {
    let msg = format!("{} is not installed", package);
    print("red", &msg, Emoji("", ""))?;
    let cmd = format!("sudo {} install {} -y", package_manager, package);
    let process = async_command(&cmd).await;
    match process {
        Ok(_) => {
            let msg = format!("Installed {}", package);
            print("green", &msg, Emoji("", ""))?;
            Ok(())
        }
        Err(e) => Err(anyhow!("Failed installing {} with error: {}", package, e)),
    }
}

pub async fn install_mac_packages() -> Result<()> {
    Ok(())
}
