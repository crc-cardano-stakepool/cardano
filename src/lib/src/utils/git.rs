use crate::{
    async_command, async_command_pipe, async_user_command, check_env, check_latest_version, get_component_path, set_env, CARDANO_NODE_URL,
};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::path::Path;

pub async fn check_repo(url: &str, absolute_path: &str) -> Result<()> {
    log::info!("Cheking if {absolute_path} is a repository");
    if Path::new(absolute_path).is_dir() {
        log::debug!("{absolute_path} exists");
        let repo_git_path = format!("{absolute_path}/.git");
        if !Path::new(&repo_git_path).is_dir() {
            log::debug!("{absolute_path} is a not git repository");
            let cmd = format!("$(ls -A {absolute_path})");
            let directory_content = async_command_pipe(&cmd).await?;
            if directory_content.is_empty() {
                return clone_repo(url, absolute_path).await;
            }
            log::debug!("{absolute_path} is not empty, skipping a clone");
        }
        log::debug!("{absolute_path} is a git repository, skipping a clone");
    }
    log::debug!("{absolute_path} does not exist, cloning into it");
    clone_repo(url, absolute_path).await
}

pub async fn checkout_latest_release(component: &str) -> Result<()> {
    log::info!("Checking out the latest release of {component}");
    let version = check_latest_version(component).await?;
    let path = get_component_path(component)?;
    let cmd = format!("cd {path} && git checkout tags/{version}");
    fetch_tags(component).await?;
    async_command(&cmd).await?;
    Ok(())
}

pub async fn clone_component(component: &str) -> Result<()> {
    log::info!("Cloning {component}");
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
        check_repo(url, &cardano_component_dir).await?;
        return checkout_latest_release(component).await;
    }
    Err(anyhow!("Failed cloning {component} repository"))
}

pub async fn clone_repo(url: &str, destination_path: &str) -> Result<()> {
    log::info!("Cloning repo to {destination_path}");
    let cmd = format!("git clone {url} {destination_path}");
    async_command(&cmd).await?;
    Ok(())
}

pub async fn fetch_tags(component: &str) -> Result<()> {
    log::info!("Fetching the latest tags of the {component} source reposity of");
    let path = get_component_path(component)?;
    let cmd = format!("cd {path} && git fetch --all --recurse-submodules --tags");
    async_user_command(&cmd).await?;
    Ok(())
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
    async fn test_check_latest_version() -> Result<()> {
        let version = check_latest_version("cardano-node").await?;
        assert_eq!(version, "1.35.0");
        Ok(())
    }
    #[tokio::test]
    #[ignore]
    async fn test_check_installed_version() {
        unimplemented!();
    }
}
