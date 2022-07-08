use crate::{
    absolute_ref_path_to_string, async_command, async_command_pipe,
    check_cabal, check_env, check_ghc, check_ghcup, check_libsodium,
    check_project_file, check_secp256k1, clone_component, copy_binary,
    get_bin_path, get_ghc_version, get_project_file, proceed,
    process_success_inherit, read_setting, set_component_dir, set_confirm,
    setup_packages, update_cabal, update_project_file, ShellConfig,
};
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Component {
    Node,
    Cli,
    Wallet,
    Address,
    Bech32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CardanoComponent {
    component: Component,
    name: String,
    installed_version: String,
    latest_version: String,
    source_path: PathBuf,
    bin_path: PathBuf,
    url: String,
    release_url: String,
}

impl CardanoComponent {
    fn new(component: Component) -> Self {
        let name = component_to_string(component);
        let installed_version = check_installed_version(component).unwrap();
        let latest_version = check_latest_version(component).unwrap();
        let source_path = get_component_path(component).unwrap();
        let bin_path = get_bin_path(&name).unwrap();
        let url = get_component_url(component);
        let release_url = get_component_release_url(component);
        Self {
            component,
            name,
            installed_version,
            latest_version,
            source_path,
            bin_path,
            url,
            release_url,
        }
    }
}
impl Default for CardanoComponent {
    fn default() -> Self {
        Self::new(Component::Node)
    }
}

pub fn check_latest_component(
    component: Component,
    confirm: bool,
) -> Result<()> {
    let component_str = component_to_string(component);
    log::info!("Checking {component_str}");
    if !is_component_installed(component)? {
        return install_latest_component(component, confirm);
    }
    let installed = check_installed_version(component)?;
    let latest = check_latest_version(component)?;
    if installed.eq(&latest) {
        let component = component_to_string(component);
        log::info!("The {component} binary is already up to date!");
        return Ok(());
    }
    install_latest_component(component, confirm)
}

pub fn install_latest_component(
    component: Component,
    confirm: bool,
) -> Result<()> {
    set_confirm(confirm);
    setup_component(component)?;
    let component_str = component_to_string(component);
    let msg =
        format!("Do you want to install the latest {component_str} binary?");
    if !confirm && proceed(&msg)? {
        return install_component(component);
    }
    install_component(component)
}

pub fn get_component_release_url(component: Component) -> String {
    match component {
        Component::Address => {
            let component = component_to_string(component);
            let url = "https://api.github.com/repos/input-output-hk/cardano-addresses/releases/latest";
            log::debug!("{component} release url: {url}");
            url.to_string()
        }
        _ => {
            let component = component_to_string(component);
            let url = format!("https://api.github.com/repos/input-output-hk/{component}/releases/latest");
            log::debug!("{component} release url: {url}");
            url
        }
    }
}

pub fn get_component_url(component: Component) -> String {
    let component = component_to_string(component);
    let url = format!("https://github.com/input-output-hk/{component}.git");
    log::debug!("{component} git url: {url}");
    url
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

pub fn check_install_success(component: Component) -> Result<()> {
    match component {
        Component::Node | Component::Cli => {
            let version = check_installed_version(component)?;
            let component = component_to_string(component);
            log::info!("Successfully installed {component} v{version}");
            check_installed_version(Component::Cli)?;
        }
        Component::Address | Component::Bech32 => {
            let version = check_installed_version(component)?;
            let component = component_to_string(component);
            log::info!("Successfully installed {component} v{version}");
        }
        Component::Wallet => {
            let version = check_installed_version(component)?;
            let component = component_to_string(component);
            log::info!("Successfully installed {component} {version}");
        }
    }
    Ok(())
}

pub fn is_component_installed(component: Component) -> Result<bool> {
    let bin = component_to_string(component);
    log::debug!("Checking if {bin} is already installed");
    let install_dir = read_setting("install_dir")?;
    let mut path = PathBuf::from(install_dir);
    path.push(&bin);
    Ok(path.exists())
}

pub fn component_to_string(component: Component) -> String {
    match component {
        Component::Node => "cardano-node".to_string(),
        Component::Cli => "cardano-cli".to_string(),
        Component::Wallet => "cardano-wallet".to_string(),
        Component::Address => "cardano-address".to_string(),
        Component::Bech32 => "bech32".to_string(),
    }
}

pub fn match_component(component: &str) -> Component {
    match component {
        "cardano-node" => Component::Node,
        "cardano-cli" => Component::Cli,
        "cardano-wallet" => Component::Wallet,
        "cardano-address" => Component::Address,
        "bech32" => Component::Bech32,
        _ => {
            log::error!("Unknown component!");
            panic!("Mismatched component")
        }
    }
}

pub fn check_installed_version(component: Component) -> Result<String> {
    let component_str = component_to_string(component);
    log::debug!("Checking installed version of {component_str}");
    let component_bin_path = get_bin_path(&component_str)?;
    let path = absolute_ref_path_to_string(component_bin_path)?;
    let cmd = match component {
        Component::Wallet | Component::Address => {
            format!("{path} version | awk '{{print $1}}'")
        }
        Component::Node | Component::Cli => {
            format!("{path} --version | awk '{{print $2}}' | head -n1")
        }
        Component::Bech32 => format!("{path} --version"),
    };
    let version = async_command_pipe(&cmd)?;
    let version = String::from(version.trim());
    Ok(version)
}

pub fn check_latest_version(component: Component) -> Result<String> {
    let component_str = component_to_string(component);
    log::debug!("Checking latest {component_str} version");
    let cmd = match component {
        Component::Wallet => {
            let path = set_component_dir(component)?;
            format!("cd {path} && git describe --tags --abbrev=0")
        }
        _ => {
            let url = get_component_release_url(component);
            format!("curl -s {url} | jq -r .tag_name")
        }
    };
    let response = async_command_pipe(&cmd)?;
    let response = String::from(response.trim());
    log::debug!("The latest version of {component_str} is {response}");
    Ok(response)
}

pub fn check_installed_component(component: Component) -> Result<()> {
    if !is_component_installed(component)? {
        return install_component(component);
    }
    install_if_not_up_to_date(component)?;
    Ok(())
}

pub fn install_component(component: Component) -> Result<()> {
    build_component(component)?;
    copy_binary(component)?;
    check_install_success(component)?;
    ShellConfig::source_shell()
}

pub fn install_if_not_up_to_date(component: Component) -> Result<()> {
    let installed = check_installed_version(component)?;
    let latest = check_latest_version(component)?;
    if !installed.eq(&latest) {
        return install_component(component);
    }
    let component = component_to_string(component);
    log::info!("Latest {component} {installed} is already installed");
    Ok(())
}

pub fn build_component(component: Component) -> Result<()> {
    let component_str = component_to_string(component);
    log::info!("Building {component_str}");
    clone_component(component)?;
    let ghc_version = get_ghc_version()?;
    let cabal = check_env("CABAL_BIN")?;
    let cabal = PathBuf::from(&cabal);
    let project_file = get_project_file(component)?;
    let path = get_component_path(component)?;
    update_cabal(&path, &cabal)?;
    check_project_file(&project_file)?;
    configure_build(&ghc_version, &path, &cabal)?;
    match component {
        Component::Node => {
            update_project_file(&project_file)?;
            build(component, &path, &cabal)
        }
        _ => build(component, &path, &cabal),
    }
}

pub fn configure_build<P: AsRef<Path>>(
    ghc_version: &str,
    path: P,
    cabal: P,
) -> Result<()> {
    log::info!("Configuring build");
    let ghc = check_env("GHC_BIN")?;
    let path = absolute_ref_path_to_string(&path)?;
    let cabal = absolute_ref_path_to_string(&cabal)?;
    let cmd = format!(
        "cd {path} && {cabal} configure --with-compiler={ghc}-{ghc_version}"
    );
    async_command(&cmd)?;
    Ok(())
}

pub fn build<P: AsRef<Path>>(
    component: Component,
    path: P,
    cabal: P,
) -> Result<()> {
    let component = component_to_string(component);
    log::info!("Building {component}");
    let path = absolute_ref_path_to_string(&path)?;
    let cabal = absolute_ref_path_to_string(&cabal)?;
    let cmd = format!("cd {path} && {cabal} build all");
    if process_success_inherit(&cmd)? {
        log::debug!("Successfully built {component}");
        return Ok(());
    }
    Err(anyhow!("Failed building {component}"))
}

pub fn setup_component(component: Component) -> Result<()> {
    log::info!("Setting up the system with build dependencies");
    setup_packages()?;
    ShellConfig::setup_shell()?;
    check_component_dependencies(component)?;
    Ok(())
}

pub fn check_component_dependencies(component: Component) -> Result<()> {
    log::info!("Checking build dependencies");
    match component {
        Component::Node => {
            check_ghcup()?;
            check_ghc()?;
            check_cabal()?;
            check_libsodium()?;
            check_secp256k1()
        }
        _ => {
            check_ghcup()?;
            check_ghc()?;
            check_cabal()
        }
    }
}

pub fn uninstall_component(component: Component) -> Result<()> {
    let component = component_to_string(component);
    log::info!("Uninstalling {component}");
    log::warn!("Not yet implemented");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uninstall_component() {
        let result = uninstall_component(Component::Node).is_ok();
        assert!(result);
    }
}
