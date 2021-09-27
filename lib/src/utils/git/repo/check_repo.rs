use crate::{async_command_pipe, clone_repo, print};
use anyhow::Result;
use std::path::Path;

pub async fn check_repo(url: &str, absolute_path: &str, repo_name: &str) -> Result<()> {
    if Path::new(absolute_path).is_dir() {
        let repo_git_path = format!("{}/.git", absolute_path);
        if !Path::new(&repo_git_path).is_dir() {
            let cmd = format!("$(ls -A {})", absolute_path);
            println!("{}", cmd);
            let directory_content = async_command_pipe(&cmd).await?;
            if directory_content.is_empty() {
                clone_repo(url, absolute_path, repo_name).await?;
            } else {
                let msg = "Can't clone into directory, directory is not empty";
                print("red", msg)?;
            }
        } else {
            let msg = format!("{} repository found", repo_name);
            print("green", &msg)?;
        }
    } else {
        let msg = format!("{} directory not found", repo_name);
        print("", &msg)?;
        clone_repo(url, absolute_path, repo_name).await?;
    }
    Ok(())
}