use crate::{
    async_command, async_command_pipe, async_user_command, check_env, check_user, chownr, get_component_path, print,
    set_env, CARDANO_NODE_URL,
};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use nix::NixPath;
use std::path::{Path, PathBuf};

pub async fn check_installed_version(component: &str) -> Result<String> {
    let component_bin_path = get_bin_path(component).await?;
    let cmd = format!("{component_bin_path} --version | awk {} | head -n1", "'{print $2}'");
    let version = async_command_pipe(&cmd).await?;
    let installed_version: String = String::from(version.trim());
    let msg = format!("{component} (v{installed_version})");
    print("green", &msg)?;
    Ok(installed_version)
}

pub async fn check_latest_version(component: &str) -> Result<String> {
    let release_url = format!("https://api.github.com/repos/input-output-hk/{component}/releases/latest");
    let cmd = format!("curl -s {release_url} | jq -r .tag_name");
    let response = async_command_pipe(&cmd).await?;
    Ok(String::from(response.trim()))
}

pub async fn check_repo(url: &str, absolute_path: impl AsRef<Path>, repo_name: &str) -> Result<()> {
    let path = absolute_path.as_ref();
    if path.is_absolute() && path.is_dir() {
        let mut git_repo_path = PathBuf::from(path);
        git_repo_path.push(".git");
        if !git_repo_path.is_dir() {
            if path.is_empty() {
                clone_repo(url, absolute_path, repo_name).await
            } else {
                print("red", "Can't clone into directory, directory is not empty")
            }
        } else {
            let msg = format!("{repo_name} repository found");
            print("green", &msg)
        }
    } else {
        clone_repo(url, absolute_path, repo_name).await
    }
}

pub async fn checkout_latest_release(component: &str) -> Result<()> {
    let version = check_latest_version(component).await?;
    let msg = format!("Checking out latest {component} release (v{version})");
    let path = get_component_path(component).await?;
    let cmd = format!("cd {path} && git checkout tags/{version}");
    fetch_tags(component).await?;
    print("", &msg)?;
    async_user_command(&cmd).await?;
    chownr(&path)
}

pub async fn clone_component(component: &str) -> Result<()> {
    let url = match component {
        "cardano-node" => Ok(CARDANO_NODE_URL),
        _ => Err(anyhow!("Unknown component {component}")),
    };
    if let Ok(url) = url {
        let work_dir = check_env("WORK_DIR")?;
        let cardano_component_dir = format!("{work_dir}/{component}");
        let env_name = format!("{component}-dir");
        let converted = env_name.to_case(Case::UpperSnake);
        set_env(&converted, &cardano_component_dir);
        check_repo(url, &cardano_component_dir, component).await?;
        checkout_latest_release(component).await
    } else {
        Err(anyhow!("Failed cloning {component} repository"))
    }
}

pub async fn clone_repo(url: &str, destination_path: impl AsRef<Path>, repo_name: &str) -> Result<()> {
    let path = destination_path.as_ref();
    if let Some(str_path) = path.to_str() {
        if !path.exists() {
            return Err(anyhow!("Invalid path: {str_path}"));
        }
    }
    if let Some(destination_path) = path.to_str() {
        let msg = format!("Cloning {repo_name} repository to {destination_path}");
        print("", &msg)?;
        let cmd = format!("git clone {url} {destination_path}");
        async_command(&cmd).await?;
        let msg = format!("Successfully cloned {repo_name} repository to {destination_path}");
        chownr(destination_path)?;
        print("green", &msg)
    } else {
        return Err(anyhow!("Failed to clone repo {repo_name}"));
    }
}

pub async fn fetch_tags(component: &str) -> Result<()> {
    let path = get_component_path(component).await?;
    let cmd = format!("cd {path} && git fetch --all --recurse-submodules --tags");
    async_user_command(&cmd).await?;
    print("green", "Successfully fetched tags")
}

pub async fn get_bin_path(bin: &str) -> Result<String> {
    let user = check_user().await?;
    let path = format!("/home/{user}/.local/bin/{bin}");
    Ok(path)
}

pub async fn is_bin_installed(bin: &str) -> Result<bool> {
    let user = check_user().await?;
    let file = format!("/home/{user}/.local/bin/{bin}");
    Ok(Path::new(&file).exists())
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
    async fn test_check_repo() -> Result<()> {
        let path = Path::new("/home/clay/.cardano/cardano-node");
        let repo_name = "cardano-node";
        check_repo(CARDANO_NODE_URL, path, repo_name).await?;
        Ok(())
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
