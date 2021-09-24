use super::color::print;
use super::config_map::REPO_URLS;
use super::env::*;
use super::fs::change_dir;
use super::process::{async_command, async_command_pipe};
use anyhow::{anyhow, Result};
use console::Emoji;
use convert_case::{Case, Casing};
use std::path::Path;
pub async fn check_repository(url: &str, absolute_path: &str, repo_name: &str) -> Result<()> {
    if Path::new(absolute_path).is_dir() {
        let msg = format!("{} found, checking for git repository", absolute_path);
        print("", &msg, Emoji("", ""))?;
        let repo_git_path = format!("{}/.git", absolute_path);
        if !Path::new(&repo_git_path).is_dir() {
            let msg = format!(
                "{} directory exists and is not a git repository, checking if it's empty",
                absolute_path
            );
            print("", &msg, Emoji("", ""))?;
            let cmd = format!("$(ls -A {})", absolute_path);
            println!("{}", cmd);
            let directory_content = async_command_pipe(&cmd).await?;
            if directory_content.is_empty() {
                let msg = format!("{} is empty, cloning into it", absolute_path);
                print("", &msg, Emoji("", ""))?;
                clone_repository(url, absolute_path, repo_name).await?;
            } else {
                let msg = "Can't clone into directory, directory is not empty";
                print("red", msg, Emoji("", ""))?;
            }
        } else {
            let msg = format!("{} repository found", repo_name);
            print("green", &msg, Emoji("", ""))?;
        }
    } else {
        let msg = format!("{} directory not found", repo_name);
        print("green", &msg, Emoji("", ""))?;
        clone_repository(url, absolute_path, repo_name).await?;
    }
    Ok(())
}

pub async fn clone_repository(url: &str, destination_path: &str, repo_name: &str) -> Result<()> {
    let work_dir = check_env("WORK_DIR")?;
    change_dir(&work_dir).await?;
    let msg = format!("Cloning {} repository to {}", repo_name, destination_path);
    print("", &msg, Emoji("", ""))?;
    let cmd = format!("git clone {} {}", url, destination_path);
    async_command(&cmd).await?;
    let msg = format!("Successfully cloned {} repository to {}", repo_name, destination_path);
    print("green", &msg, Emoji("", ""))?;
    Ok(())
}

pub async fn clone_cardano_repository(component: &str) -> Result<()> {
    if let Some(url) = REPO_URLS.get(component) {
        let work_dir = check_env("WORK_DIR")?;
        let cardano_component_dir = format!("{}/{}", work_dir, component);
        let env_name = format!("{}-dir", component);
        let converted = env_name.to_case(Case::UpperSnake);
        set_env(&converted, &cardano_component_dir);
        check_repository(url, &cardano_component_dir, "cardano-node").await?;
        Ok(())
    } else {
        let msg = format!("Failed cloning {} repository", component);
        Err(anyhow!(msg))
    }
}
