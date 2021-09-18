use super::color::print;
use super::process::async_command;
use super::user::check_user;
use anyhow::Result;
use console::Emoji;
use std::path::Path;
use tokio::fs::create_dir_all;

pub async fn check_directory(dir_name: &str, absolute_path: &str) -> Result<()> {
    let msg = format!("Checking for {} directory in {}", dir_name, absolute_path);
    print("", &msg, Emoji("", ""))?;
    if Path::new(absolute_path).is_dir() {
        let msg = format!("{} {}", dir_name, "directory found, skipped creating");
        print("green", &msg, Emoji("", ""))?;
    } else {
        create_directory(dir_name, absolute_path).await?;
    }
    Ok(())
}

pub async fn create_directory(dir_name: &str, absolute_path: &str) -> Result<()> {
    let msg = format!("Creating directory {} in {}", dir_name, absolute_path);
    let user = check_user().await?;
    let user = user.trim();
    print("", &msg, Emoji("", ""))?;
    create_dir_all(absolute_path).await?;
    chownr(&user, &user, absolute_path).await?;
    change_dir(absolute_path).await?;
    Ok(())
}

pub async fn chownr(user: &str, group: &str, absolute_path: &str) -> Result<()> {
    let cmd = format!("chown -R {}:{} {}", user, group, absolute_path);
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
