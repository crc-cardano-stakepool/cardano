use crate::{async_command_pipe, clone_repo, print};
use anyhow::Result;
use std::path::Path;

pub async fn check_repo(url: &str, absolute_path: &str, repo_name: &str) -> Result<()> {
    if Path::new(absolute_path).is_dir() {
        let msg = format!("{} found, checking for git repository", absolute_path);
        print("", &msg)?;
        let repo_git_path = format!("{}/.git", absolute_path);
        if !Path::new(&repo_git_path).is_dir() {
            let msg = format!(
                "{} directory exists and is not a git repository, checking if it's empty",
                absolute_path
            );
            print("", &msg)?;
            let cmd = format!("$(ls -A {})", absolute_path);
            println!("{}", cmd);
            let directory_content = async_command_pipe(&cmd).await?;
            if directory_content.is_empty() {
                let msg = format!("{} is empty, cloning into it", absolute_path);
                print("", &msg)?;
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
        print("green", &msg)?;
        clone_repo(url, absolute_path, repo_name).await?;
    }
    Ok(())
}
