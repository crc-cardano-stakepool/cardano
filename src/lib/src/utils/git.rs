use crate::{
    absolute_ref_path_to_string, async_command, check_latest_version,
    check_work_dir, component_to_string, get_component_path, get_component_url,
    path_to_string, set_env, Component,
};
use anyhow::Result;
use convert_case::{Case, Casing};
use std::path::{Path, PathBuf};

pub fn check_repo<P: AsRef<Path>>(url: &str, absolute_path: P) -> Result<()> {
    let path = path_to_string(absolute_path.as_ref())?;
    let mut path_buf = PathBuf::from(absolute_path.as_ref());
    log::debug!("Checking if {path} is a repository");
    if !path_buf.is_dir() {
        log::debug!("{path} does not exist, cloning into it");
        return clone_repo(url, absolute_path);
    }
    log::debug!("{path} exists");
    path_buf.push(".git");
    if path_buf.is_dir() {
        log::debug!("{path} is a git repository, skipping a clone");
        return Ok(());
    }
    path_buf.pop();
    if path_buf.read_dir()?.next().is_none() {
        return clone_repo(url, absolute_path);
    }
    Ok(())
}

pub fn checkout_latest_release(component: Component) -> Result<()> {
    let version = check_latest_version(component)?;
    let path = get_component_path(component)?;
    let path = absolute_ref_path_to_string(&path)?;
    let cmd = format!("cd {path} && git checkout tags/{version}");
    fetch_tags(component)?;
    let component = component_to_string(component);
    log::debug!("Checking out the latest release of {component}");
    async_command(&cmd)?;
    Ok(())
}

pub fn set_component_dir(component: Component) -> Result<String> {
    match component {
        Component::Cli => {
            let component = component_to_string(component);
            log::debug!("Setting the directory for {component}");
            let mut work_dir = check_work_dir()?.as_ref().to_path_buf();
            work_dir.push("cardano-node");
            let component_dir = path_to_string(&work_dir)?;
            let env_name = format!("{component}-dir");
            let converted = env_name.to_case(Case::UpperSnake);
            set_env(&converted, &component_dir);
            Ok(component_dir)
        }
        Component::Address => {
            let component = component_to_string(component);
            log::debug!("Setting the directory for {component}");
            let mut work_dir = check_work_dir()?.as_ref().to_path_buf();
            work_dir.push("cardano-addresses");
            let component_dir = path_to_string(&work_dir)?;
            let env_name = format!("{component}-dir");
            let converted = env_name.to_case(Case::UpperSnake);
            set_env(&converted, &component_dir);
            Ok(component_dir)
        }
        _ => {
            let component = component_to_string(component);
            log::debug!("Setting the directory for {component}");
            let mut work_dir = check_work_dir()?.as_ref().to_path_buf();
            work_dir.push(&component);
            let component_dir = path_to_string(&work_dir)?;
            let env_name = format!("{component}-dir");
            let converted = env_name.to_case(Case::UpperSnake);
            set_env(&converted, &component_dir);
            Ok(component_dir)
        }
    }
}

pub fn clone_component(component: Component) -> Result<()> {
    let component_dir = set_component_dir(component)?;
    let url = get_component_url(component);
    check_repo(&url, &component_dir)?;
    checkout_latest_release(component)
}

pub fn clone_repo<P: AsRef<Path>>(
    url: &str,
    destination_path: P,
) -> Result<()> {
    let path = path_to_string(destination_path.as_ref())?;
    log::info!("Cloning repo to {path}");
    let cmd = format!("git clone {url} {path}");
    async_command(&cmd)?;
    Ok(())
}

pub fn fetch_tags(component: Component) -> Result<()> {
    let path = get_component_path(component)?;
    let path = absolute_ref_path_to_string(&path)?;
    let cmd =
        format!("cd {path} && git fetch --all --recurse-submodules --tags");
    let component = component_to_string(component);
    log::info!(
        "Fetching the latest tags of the {component} source reposity of"
    );
    async_command(&cmd)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::CARDANO_NODE_VERSION;

    use super::*;

    #[test]
    fn test_check_latest_version() -> Result<()> {
        let version = check_latest_version(Component::Node)?;
        assert_eq!(version, CARDANO_NODE_VERSION);
        Ok(())
    }
}
