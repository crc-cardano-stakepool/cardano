use crate::{check_package, print};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;

pub async fn install_packages(package_manager: &str, packages: &[&str]) -> Result<()> {
    print("", "Installing packages")?;
    let progress_style = ProgressStyle::default_bar()
        .template("[{bar:.white/white}] {wide_msg:.green/green}")
        .progress_chars("=>-");
    let pkgs: u64 = packages.len() as u64;
    let pb = ProgressBar::new(pkgs);
    let mut i = 1;
    pb.set_style(progress_style);
    for package in packages {
        sleep(Duration::from_millis(80));
        check_package(package_manager, package).await?;
        pb.set_message(format!("[{}/{}] {} is installed", i, pkgs, package));
        pb.inc(1);
        i += 1;
    }
    pb.finish_and_clear();
    print("green", "Successfully installed packages")?;
    Ok(())
}
