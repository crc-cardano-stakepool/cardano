use super::color::print;
use super::env::set_env;
use super::process::async_command;
use super::user::check_user;
use anyhow::Result;
use console::Emoji;
use std::path::Path;
use tokio::fs::create_dir_all;

pub async fn check_directory(dir_name: &str, absolute_path: &str) -> Result<()> {
    let msg = format!("Checking for {} in {}", dir_name, absolute_path);
    print("", &msg, Emoji("", ""))?;
    if Path::new(absolute_path).is_dir() {
        let msg = format!("Found {}, skipped creating", dir_name);
        print("green", &msg, Emoji("", ""))?;
    } else {
        create_directory(dir_name, absolute_path).await?;
    }
    Ok(())
}

pub async fn check_home_dir() -> Result<String> {
    let user = check_user().await?;
    let home_directory = format!("/home/{}", user.trim());
    set_env("RUNNER_HOME", &home_directory);
    Ok(home_directory)
}

pub async fn check_work_dir() -> Result<String> {
    let home = check_home_dir().await?;
    let install_directory = format!("{}/.cardano", home);
    set_env("WORK_DIR", &install_directory);
    Ok(install_directory)
}

pub async fn create_directory(dir_name: &str, absolute_path: &str) -> Result<()> {
    let msg = format!("Creating {} in {}", dir_name, absolute_path);
    print("", &msg, Emoji("", ""))?;
    create_dir_all(absolute_path).await?;
    chownr(absolute_path).await?;
    change_dir(absolute_path).await?;
    Ok(())
}

pub async fn chownr(absolute_path: &str) -> Result<()> {
    let user = check_user().await?;
    let user = user.trim();
    let cmd = format!("chown -R {}:{} {}", user, user, absolute_path);
    async_command(&cmd).await?;
    Ok(())
}

pub async fn change_dir(absolute_path: &str) -> Result<()> {
    let msg = format!("Changing directory to {}", absolute_path);
    let cmd = format!("cd {}", absolute_path);
    print("", &msg, Emoji("", ""))?;
    async_command(&cmd).await?;
    Ok(())
}
