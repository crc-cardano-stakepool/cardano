use crate::{check_package, pipe, print, process_success, spinner_cmd};
use anyhow::{anyhow, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::{thread::sleep, time::Duration};

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
    let msg = format!("{package} is not installed");
    print("red", &msg)?;
    let cmd = format!("sudo {package_manager} install {package} -y");
    let process = spinner_cmd(&cmd, "Installing package", "Finished installing package").await;
    match process {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed installing {package} with error: {e}")),
    }
}

pub async fn install_packages(package_manager: &str, packages: &[&str]) -> Result<()> {
    let mut i = 1;
    let pkgs: u64 = packages.len() as u64;
    let pb = ProgressBar::new(pkgs);
    let progress_style = ProgressStyle::default_bar()
        .template("{msg:.green}[{bar:.white}]")
        .progress_chars("=>-");
    pb.set_style(progress_style);
    for package in packages {
        sleep(Duration::from_millis(80));
        check_package(package_manager, package).await?;
        pb.set_message(format!("{package} is installed\n[{i}/{pkgs}]"));
        pb.inc(1);
        i += 1;
    }
    pb.finish_and_clear();
    print("green", "Successfully installed packages")
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
