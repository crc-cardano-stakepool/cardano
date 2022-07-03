use crate::{
    absolute_ref_path_to_string, async_command, async_user_command, check_env, check_latest_version, get_component_path, set_env,
    CARDANO_NODE_URL,
};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::path::{Path, PathBuf};

pub async fn check_repo<P: AsRef<Path>>(url: &str, absolute_path: P) -> Result<()> {
    let path = absolute_ref_path_to_string(&absolute_path)?;
    let mut path_buf = PathBuf::from(absolute_path.as_ref());
    log::debug!("Checking if {path} is a repository");
    if !path_buf.is_dir() {
        log::debug!("{path} does not exist, cloning into it");
        return clone_repo(url, absolute_path).await;
    }
    log::debug!("{path} exists");
    path_buf.push(".git");
    if path_buf.is_dir() {
        log::debug!("{path} is a git repository, skipping a clone");
        return Ok(());
    }
    path_buf.pop();
    if path_buf.read_dir()?.next().is_none() {
        return clone_repo(url, absolute_path).await;
    }
    Ok(())
}

pub async fn checkout_latest_release(component: &str) -> Result<()> {
    log::debug!("Checking out the latest release of {component}");
    let version = check_latest_version(component).await?;
    let path = get_component_path(component)?;
    let path = absolute_ref_path_to_string(&path)?;
    let cmd = format!("cd {path} && git checkout tags/{version}");
    fetch_tags(component).await?;
    async_command(&cmd).await?;
    Ok(())
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
        check_repo(url, &cardano_component_dir).await?;
        return checkout_latest_release(component).await;
    }
    Err(anyhow!("Failed cloning {component} repository"))
}

pub async fn clone_repo<P: AsRef<Path>>(url: &str, destination_path: P) -> Result<()> {
    let path = absolute_ref_path_to_string(&destination_path)?;
    log::info!("Cloning repo to {path}");
    let cmd = format!("git clone {url} {path}");
    async_command(&cmd).await?;
    Ok(())
}

pub async fn fetch_tags(component: &str) -> Result<()> {
    log::info!("Fetching the latest tags of the {component} source reposity of");
    let path = get_component_path(component)?;
    let path = absolute_ref_path_to_string(&path)?;
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
