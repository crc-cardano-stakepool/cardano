use crate::{
    async_command, async_command_pipe, async_user_command, change_dir, check_env, check_user, chownr, file_exists,
    get_component_path, print, set_env, CARDANO_NODE_URL,
};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::path::Path;

pub async fn check_installed_version(component: &str) -> Result<String> {
    let component_bin_path = get_bin_path(component).await?;
    let cmd = format!("{} --version | awk {} | head -n1", component_bin_path, "'{print $2}'");
    let version = async_command_pipe(&cmd).await?;
    let installed_version: String = String::from(version.trim());
    let msg = format!("{} (v{})", component, installed_version);
    print("green", &msg)?;
    Ok(installed_version)
}

pub async fn check_latest_version(component: &str) -> Result<String> {
    let release_url = format!(
        "https://api.github.com/repos/input-output-hk/{}/releases/latest",
        component
    );
    let cmd = format!("curl -s {} | jq -r .tag_name", release_url);
    let response = async_command_pipe(&cmd).await?;
    Ok(String::from(response.trim()))
}

pub async fn check_repo(url: &str, absolute_path: &str, repo_name: &str) -> Result<()> {
    if Path::new(absolute_path).is_dir() {
        let repo_git_path = format!("{}/.git", absolute_path);
        if !Path::new(&repo_git_path).is_dir() {
            let cmd = format!("$(ls -A {})", absolute_path);
            let directory_content = async_command_pipe(&cmd).await?;
            if directory_content.is_empty() {
                clone_repo(url, absolute_path, repo_name).await
            } else {
                let msg = "Can't clone into directory, directory is not empty";
                print("red", msg)
            }
        } else {
            let msg = format!("{} repository found", repo_name);
            print("green", &msg)
        }
    } else {
        clone_repo(url, absolute_path, repo_name).await
    }
}

pub async fn checkout_latest_release(component: &str) -> Result<()> {
    let version = check_latest_version(component).await?;
    let msg = format!("Checking out latest {} release ({})", component, version);
    let path = get_component_path(component).await?;
    let cmd = format!("cd {} && git checkout tags/{}", path, version);
    fetch_tags(component).await?;
    print("", &msg)?;
    async_user_command(&cmd).await?;
    chownr(&path).await
}

pub async fn clone_component(component: &str) -> Result<()> {
    let url = match component {
        "cardano-node" => Ok(CARDANO_NODE_URL),
        _ => Err(anyhow!("Unknown component {}", component)),
    };
    if let Ok(url) = url {
        let work_dir = check_env("WORK_DIR")?;
        let cardano_component_dir = format!("{}/{}", work_dir, component);
        let env_name = format!("{}-dir", component);
        let converted = env_name.to_case(Case::UpperSnake);
        set_env(&converted, &cardano_component_dir);
        check_repo(url, &cardano_component_dir, component).await?;
        checkout_latest_release(component).await
    } else {
        Err(anyhow!("Failed cloning {} repository", component))
    }
}

pub async fn clone_repo(url: &str, destination_path: &str, repo_name: &str) -> Result<()> {
    let work_dir = check_env("WORK_DIR")?;
    change_dir(&work_dir).await?;
    let msg = format!("Cloning {} repository to {}", repo_name, destination_path);
    print("", &msg)?;
    let cmd = format!("git clone {} {}", url, destination_path);
    async_command(&cmd).await?;
    let msg = format!("Successfully cloned {} repository to {}", repo_name, destination_path);
    chownr(destination_path).await?;
    print("green", &msg)
}

pub async fn fetch_tags(component: &str) -> Result<()> {
    let path = get_component_path(component).await?;
    let cmd = format!("cd {} && git fetch --all --recurse-submodules --tags", path);
    async_user_command(&cmd).await?;
    print("green", "Successfully fetched tags")
}

pub async fn get_bin_path(bin: &str) -> Result<String> {
    let user = check_user().await?;
    let path = format!("/home/{}/.local/bin/{}", user, bin);
    Ok(path)
}

pub async fn is_bin_installed(bin: &str) -> Result<bool> {
    let user = check_user().await?;
    let file = format!("/home/{}/.local/bin/{}", user, bin);
    Ok(file_exists(&file))
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    #[ignore]
    async fn test_fetch_tags() {
        unimplemented!();
    }
    #[tokio::test]
    #[ignore]
    async fn test_clone_repo() {
        unimplemented!();
    }
    #[tokio::test]
    #[ignore]
    async fn test_clone_cardano_repo() {
        unimplemented!();
    }
    #[tokio::test]
    #[ignore]
    async fn test_checkout_latest_release() {
        unimplemented!();
    }
    #[tokio::test]
    #[ignore]
    async fn test_check_repo() {
        unimplemented!();
    }
    #[tokio::test]
    #[ignore]
    async fn test_check_latest_version() -> Result<()> {
        let version = check_latest_version("cardano-node").await?;
        assert_eq!(version, "1.34.1");
        Ok(())
    }
    #[tokio::test]
    #[ignore]
    async fn test_check_installed_version() {
        unimplemented!();
    }
}
