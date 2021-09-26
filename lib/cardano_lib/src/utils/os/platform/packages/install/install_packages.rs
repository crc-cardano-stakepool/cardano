use crate::{async_command, check_package, print};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;

pub async fn install_packages(package_manager: &str, packages: &[&str]) -> Result<()> {
    println!("Updating");
    let cmd = format!(
        "sudo {} update -y && sudo {} upgrade -y",
        package_manager, package_manager
    );
    async_command(&cmd).await?;
    print("green", "Finished updating")?;
    print("", "Installing packages")?;
    let spinner_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{wide_bar:.white/white}] {msg:.green/green}")
        .progress_chars("#>-");
    let pkgs: u64 = packages.len() as u64;
    let pb = ProgressBar::new(pkgs);
    let mut i = 1;
    pb.set_style(spinner_style);
    pb.set_draw_rate(100);
    for package in packages {
        sleep(Duration::from_millis(125));
        check_package(package_manager, package).await?;
        pb.set_message(format!("[{}/{}] {} is installed", i, pkgs, package));
        pb.inc(1);
        i = i+1;
    }
    pb.finish_and_clear();
    print("green", "Successfully installed packages")?;
    Ok(())
}
