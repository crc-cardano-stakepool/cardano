use crate::{chownr, async_command, change_dir, check_env, print};
use anyhow::Result;

pub async fn clone_repo(url: &str, destination_path: &str, repo_name: &str) -> Result<()> {
    let work_dir = check_env("WORK_DIR")?;
    change_dir(&work_dir).await?;
    let msg = format!("Cloning {} repository to {}", repo_name, destination_path);
    print("", &msg)?;
    let cmd = format!("git clone {} {}", url, destination_path);
    async_command(&cmd).await?;
    let msg = format!("Successfully cloned {} repository to {}", repo_name, destination_path);
    chownr(destination_path).await?;
    print("green", &msg)?;
    Ok(())
}
