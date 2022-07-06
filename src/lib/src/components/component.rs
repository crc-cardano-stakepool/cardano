use crate::{
    absolute_ref_path_to_string, async_command_pipe, build_node, build_wallet, check_env, get_bin_path, set_component_dir, install_node,
    install_wallet, read_setting, CARDANO_NODE_RELEASE_URL, CARDANO_NODE_URL, CARDANO_WALLET_RELEASE_URL, CARDANO_WALLET_URL,
};
use anyhow::Result;
use convert_case::{Case, Casing};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Component {
    Node,
    Cli,
    Wallet,
}

pub fn get_component_release_url(component: Component) -> &'static str {
    match component {
        Component::Node => CARDANO_NODE_RELEASE_URL,
        Component::Cli => CARDANO_NODE_RELEASE_URL,
        Component::Wallet => CARDANO_WALLET_RELEASE_URL,
    }
}

pub fn get_component_url(component: Component) -> &'static str {
    match component {
        Component::Node => CARDANO_NODE_URL,
        Component::Cli => CARDANO_NODE_URL,
        Component::Wallet => CARDANO_WALLET_URL,
    }
}

pub fn get_component_path(component: Component) -> Result<PathBuf> {
    let component = component_to_string(component);
    log::debug!("Checking where the binary of {component} is");
    let env = format!("{component}_dir");
    let converted = env.to_case(Case::UpperSnake);
    let path = check_env(&converted)?;
    let path = PathBuf::from(&path);
    Ok(path)
}

pub async fn check_install(component: Component) -> Result<()> {
    match component {
        Component::Node => {
            let version = check_installed_version(component).await?;
            let component = component_to_string(component);
            log::info!("Successfully installed {component} v{version}");
            check_installed_version(Component::Cli).await?;
        }
        Component::Wallet => {
            let version = check_installed_version(component).await?;
            let component = component_to_string(component);
            log::info!("Successfully installed {component} {version}");
        }
        _ => (),
    }
    Ok(())
}

pub fn is_component_installed(component: Component) -> Result<bool> {
    let bin = component_to_string(component);
    log::debug!("Checking if {bin} is already installed");
    let install_dir = read_setting("install_dir")?;
    let mut path = PathBuf::from(install_dir);
    path.push(&bin);
    Ok(path.is_file())
}

pub fn component_to_string(component: Component) -> String {
    match component {
        Component::Node => "cardano-node".to_string(),
        Component::Cli => "cardano-cli".to_string(),
        Component::Wallet => "cardano-wallet".to_string(),
    }
}

pub fn match_component(component: &str) -> Component {
    match component {
        "cardano-node" => Component::Node,
        "cardano-cli" => Component::Cli,
        "cardano-wallet" => Component::Wallet,
        _ => {
            log::error!("Unknown component!");
            panic!("Mismatched component")
        }
    }
}

pub async fn check_installed_version(component: Component) -> Result<String> {
    match component {
        Component::Wallet => {
            let component = component_to_string(component);
            let component_bin_path = get_bin_path(&component)?;
            let path = absolute_ref_path_to_string(component_bin_path)?;
            let cmd = format!("{path} version | awk '{{print $1}}'");
            log::debug!("Checking installed version of {component}");
            let version = async_command_pipe(&cmd).await?;
            let installed_version: String = String::from(version.trim());
            Ok(installed_version)
        }
        _ => {
            let component = component_to_string(component);
            let component_bin_path = get_bin_path(&component)?;
            let path = absolute_ref_path_to_string(component_bin_path)?;
            let cmd = format!("{path} --version | awk '{{print $2}}' | head -n1");
            log::debug!("Checking installed version of {component}");
            let version = async_command_pipe(&cmd).await?;
            let installed_version: String = String::from(version.trim());
            Ok(installed_version)
        }
    }
}

pub async fn check_latest_version(component: Component) -> Result<String> {
    match component {
        Component::Wallet => {
            let path = set_component_dir(component)?;
            let cmd = format!("cd {path} && git describe --tags --abbrev=0");
            let component = component_to_string(component);
            log::debug!("Checking latest {component} version");
            let response = async_command_pipe(&cmd).await?;
            let response = String::from(response.trim());
            log::debug!("The latest version of {component} is {response}");
            Ok(response)
        }
        _ => {
            let url = get_component_release_url(component);
            let component = component_to_string(component);
            log::debug!("Checking latest {component} version");
            let cmd = format!("curl -s {url} | jq -r .tag_name");
            let response = async_command_pipe(&cmd).await?;
            let response = String::from(response.trim());
            log::debug!("The latest version of {component} is {response}");
            Ok(response)
        }
    }
}

pub async fn check_installed_component(component: Component) -> Result<()> {
    if !is_component_installed(component)? {
        return install_component(component).await;
    }
    install_if_not_up_to_date(component).await?;
    Ok(())
}

pub async fn install_component(component: Component) -> Result<()> {
    match component {
        Component::Node => install_node().await,
        Component::Cli => install_node().await,
        Component::Wallet => install_wallet().await,
    }
}

async fn install_if_not_up_to_date(component: Component) -> Result<()> {
    let installed = check_installed_version(component).await?;
    let latest = check_latest_version(component).await?;
    if !installed.eq(&latest) {
        return install_component(component).await;
    }
    let component = component_to_string(component);
    log::info!("Latest {component} v{installed} is already installed");
    Ok(())
}

pub async fn build_component(component: Component) -> Result<()> {
    match component {
        Component::Node => build_node().await,
        Component::Cli => build_node().await,
        Component::Wallet => build_wallet().await,
    }
}
