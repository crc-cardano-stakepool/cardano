use super::process::{async_command_pipe, pipe};
use crate::cli::utils::color::print;
use anyhow::{anyhow, Result};
use console::Emoji;

pub async fn check_distro() -> Result<String> {
    println!("Checking distro");
    let helper_string = "'{print $2}'";
    let cmd = format!("cat /etc/*ease | grep DISTRIB_ID | awk -F '=' {}", helper_string);
    let distro = async_command_pipe(&cmd).await;
    match distro {
        Ok(distro) => {
            let msg = format!("Detected {}", distro.trim());
            print("green", &msg, Emoji("", ""))?;
            Ok(distro)
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
            print("green", "Detected Linux", Emoji("",""))?;
            let output = check_distro().await?;
            let distro = output.as_str().trim();
            install_distro_packages(distro).await?;
        }
        "darwin" | "Darwin" => {
            print("green", "Detected macOS", Emoji("",""))?;
            install_mac_packages().await?
        },
        _ => panic!("Unsupported platform: {}", platform),
    }
    Ok(())
}

pub async fn install_distro_packages(distro: &str) -> Result<()> {
    println!("Installing {} dependencies", distro);
    match distro {
        "Ubuntu" | "Debian" => {
            let package_manager = "apt";
            let packages = vec![
                "curl",
                "automake",
                "build-essential",
                "pkg-config",
                "libffi-dev",
                "libgmp-dev",
                "libssl-dev",
                "libtinfo-dev",
                "libsystemd-dev",
                "zlib1g-dev",
                "make",
                "g++",
                "tmux",
                "git",
                "jq",
                "wget",
                "libncursesw5",
                "libtool",
                "autoconf",
            ];
            install_packages(package_manager, packages).await?;
        }
        "Fedora" | "Hat" | "CentOs" => {
            let package_manager = "yum";
            let packages = vec![
                "curl",
                "git",
                "gcc",
                "gcc-c++",
                "tmux",
                "gmp-devel",
                "make",
                "tar",
                "xz",
                "wget",
                "zlib-devel",
                "libtool",
                "autoconf",
                "systemd-devel",
                "ncurses-devel",
                "ncurses-compat-libs",
            ];
            install_packages(package_manager, packages).await?;
        }
        _ => panic!("Unsupported distro: {}", distro),
    }
    Ok(())
}

pub async fn install_packages(package_manager: &str, packages: Vec<&str>) -> Result<()> {
    println!("Updating");
    let cmd = format!("{} update -y", package_manager);
    async_command_pipe(&cmd).await?;
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
    let cmd = format!("{} install {}", package_manager, package);
    let process = async_command_pipe(&cmd).await;
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
