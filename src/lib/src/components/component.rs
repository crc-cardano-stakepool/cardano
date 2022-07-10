use crate::{
    check_libsodium, check_secp256k1, proceed, Cabal, Environment, Executer,
    FileSystem, Ghc, Ghcup, Git, PlatformInfo, Settings, ShellConfig,
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
    pub component: Component,
    pub binary_name: String,
    pub source_path: PathBuf,
    pub source_url: String,
    pub release_url: String,
    pub latest_version: String,
    pub installed_version: Option<String>,
    pub bin_path: Option<PathBuf>,
}

impl CardanoComponent {
    pub fn new(component: Component) -> Self {
        let binary_name = Self::component_to_string(component);
        let source_path = Self::get_component_path(component).unwrap();
        let source_url = Self::get_component_url(component);
        let release_url = Self::get_component_release_url(component);
        let latest_version = Self::check_latest_version(component).unwrap();
        let installed_version = Self::check_installed_version(component).ok();
        let bin_path = FileSystem::get_bin_path(&binary_name).ok();
        Self {
            component,
            binary_name,
            source_path,
            source_url,
            release_url,
            latest_version,
            installed_version,
            bin_path,
        }
    }
}

impl Default for CardanoComponent {
    fn default() -> Self {
        Self::new(Component::Node)
    }
}

impl CardanoComponent {
    pub fn check_latest_component(
        component: Component,
        confirm: bool,
    ) -> Result<()> {
        let component_str = Self::component_to_string(component);
        log::info!("Checking {component_str}");
        if !Self::is_component_installed(component)? {
            return Self::install_latest_component(component, confirm);
        }
        let installed = Self::check_installed_version(component)?;
        let latest = Self::check_latest_version(component)?;
        if installed.eq(&latest) {
            let component = Self::component_to_string(component);
            log::info!("The {component} binary is already up to date!");
            return Ok(());
        }
        Self::install_latest_component(component, confirm)
    }

    pub fn check_installed_component(component: Component) -> Result<()> {
        if !Self::is_component_installed(component)? {
            return Self::install_component(component);
        }
        Self::install_if_not_up_to_date(component)
    }

    pub fn install_component(component: Component) -> Result<()> {
        Self::build_component(component)?;
        FileSystem::copy_binary(component)?;
        Self::check_install_success(component)?;
        ShellConfig::source_shell()
    }

    pub fn install_if_not_up_to_date(component: Component) -> Result<()> {
        let installed = Self::check_installed_version(component)?;
        let latest = Self::check_latest_version(component)?;
        if !installed.eq(&latest) {
            return Self::install_component(component);
        }
        let component = Self::component_to_string(component);
        log::info!("Latest {component} {installed} is already installed");
        Ok(())
    }

    pub fn install_latest_component(
        component: Component,
        confirm: bool,
    ) -> Result<()> {
        Environment::set_confirm(confirm);
        Self::setup_component(component)?;
        let component_str = Self::component_to_string(component);
        let msg = format!(
            "Do you want to install the latest {component_str} binary?"
        );
        if !confirm && proceed(&msg)? {
            return Self::install_component(component);
        }
        Self::install_component(component)
    }

    pub fn setup_component(component: Component) -> Result<()> {
        log::info!("Setting up the system with build dependencies");
        let platform = PlatformInfo::new();
        platform.setup_packages()?;
        ShellConfig::setup_shell()?;
        Self::check_component_dependencies(component)
    }

    pub fn check_component_dependencies(component: Component) -> Result<()> {
        log::info!("Checking build dependencies");
        match component {
            Component::Node => {
                Ghcup::check_ghcup()?;
                Ghc::check()?;
                Cabal::check()?;
                check_libsodium()?;
                check_secp256k1()
            }
            _ => {
                Ghcup::check_ghcup()?;
                Ghc::check()?;
                Cabal::check()
            }
        }
    }

    pub fn check_installed_version(component: Component) -> Result<String> {
        let component_str = Self::component_to_string(component);
        log::debug!("Checking installed version of {component_str}");
        let component_bin_path = FileSystem::get_bin_path(&component_str)?;
        let path = FileSystem::absolute_ref_path_to_string(component_bin_path)?;
        let cmd = match component {
            Component::Wallet | Component::Address => {
                format!("{path} version | awk '{{print $1}}'")
            }
            Component::Node | Component::Cli => {
                format!("{path} --version | awk '{{print $2}}' | head -n1")
            }
            Component::Bech32 => format!("{path} --version"),
        };
        Executer::capture(&cmd)
    }

    pub fn check_latest_version(component: Component) -> Result<String> {
        let component_str = Self::component_to_string(component);
        log::debug!("Checking latest {component_str} version");
        let cmd = match component {
            Component::Wallet => {
                let url = Self::get_component_url(component);
                let dir = Git::set_component_dir(component)?;
                let path = Self::get_component_path(component)?;
                Git::check_repo(&url, path)?;
                format!("cd {dir} && git describe --tags --abbrev=0")
            }
            _ => {
                let url = Self::get_component_release_url(component);
                format!("curl -s {url} | jq -r .tag_name")
            }
        };
        Executer::capture(&cmd).map(|response| {
            log::debug!("The latest version of {component_str} is {response}");
            response
        })
    }

    pub fn get_component_release_url(component: Component) -> String {
        match component {
            Component::Address => {
                let component = Self::component_to_string(component);
                let url = "https://api.github.com/repos/input-output-hk/cardano-addresses/releases/latest";
                log::debug!("{component} release url: {url}");
                url.to_string()
            }
            Component::Cli => {
                let component = Self::component_to_string(component);
                let url = "https://api.github.com/repos/input-output-hk/cardano-node/releases/latest";
                log::debug!("{component} release url: {url}");
                url.to_string()
            }
            _ => {
                let component = Self::component_to_string(component);
                let url = format!("https://api.github.com/repos/input-output-hk/{component}/releases/latest");
                log::debug!("{component} release url: {url}");
                url
            }
        }
    }

    pub fn get_component_url(component: Component) -> String {
        match component {
            Component::Address => {
                let component = Self::component_to_string(component);
                let url =
                    "https://github.com/input-output-hk/cardano-addresses.git";
                log::debug!("{component} git url: {url}");
                url.to_string()
            }
            Component::Cli => {
                let component = Self::component_to_string(component);
                let url = "https://github.com/input-output-hk/cardano-node.git";
                log::debug!("{component} git url: {url}");
                url.to_string()
            }
            _ => {
                let component = Self::component_to_string(component);
                let url = format!(
                    "https://github.com/input-output-hk/{component}.git"
                );
                log::debug!("{component} git url: {url}");
                url
            }
        }
    }

    pub fn get_component_path(component: Component) -> Result<PathBuf> {
        Git::set_component_dir(component).unwrap();
        let component = Self::component_to_string(component);
        log::debug!("Checking where the binary of {component} is");
        let env = format!("{component}_dir");
        let converted = env.to_case(Case::UpperSnake);
        Environment::check_env(&converted)
            .map(PathBuf::from)
            .map(Ok)?
    }

    pub fn is_component_installed(component: Component) -> Result<bool> {
        let bin = Self::component_to_string(component);
        log::debug!("Checking if {bin} is already installed");
        Settings::read("install_dir")
            .map(PathBuf::from)
            .map(|mut path| {
                path.push(&bin);
                Ok(path.exists())
            })?
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

    pub fn build_component(component: Component) -> Result<()> {
        let component_str = Self::component_to_string(component);
        log::info!("Building {component_str}");
        Git::clone_component(component)?;
        let ghc_version = Ghc::get_version()?;
        let cabal = Environment::check_env("CABAL_BIN").map(PathBuf::from)?;
        let project_file = FileSystem::get_project_file(component)?;
        let path = Self::get_component_path(component)?;
        Cabal::update(&path, &cabal)?;
        FileSystem::check_project_file(&project_file)?;
        Self::configure_build(&ghc_version, &path, &cabal)?;
        match component {
            Component::Node => {
                FileSystem::update_project_file(&project_file)?;
                Self::build(component, &path, &cabal)
            }
            _ => Self::build(component, &path, &cabal),
        }
    }

    pub fn configure_build<P: AsRef<Path>>(
        ghc_version: &str,
        path: P,
        cabal: P,
    ) -> Result<()> {
        log::info!("Configuring build");
        let ghc = Environment::check_env("GHC_BIN")?;
        let path = FileSystem::absolute_ref_path_to_string(&path)?;
        let cabal = FileSystem::absolute_ref_path_to_string(&cabal)?;
        let cmd = format!(
        "cd {path} && {cabal} configure --with-compiler={ghc}-{ghc_version}"
    );
        Executer::exec(&cmd)?;
        Ok(())
    }

    pub fn build<P: AsRef<Path>>(
        component: Component,
        path: P,
        cabal: P,
    ) -> Result<()> {
        let component = Self::component_to_string(component);
        log::info!("Building {component}");
        let path = FileSystem::absolute_ref_path_to_string(&path)?;
        let cabal = FileSystem::absolute_ref_path_to_string(&cabal)?;
        let cmd = format!("cd {path} && {cabal} build all");
        if Executer::capture_success(&cmd)? {
            log::debug!("Successfully built {component}");
            return Ok(());
        }
        Err(anyhow!("Failed building {component}"))
    }

    pub fn check_install_success(component: Component) -> Result<()> {
        let version = Self::check_installed_version(component)?;
        let component_str = Self::component_to_string(component);
        match component {
            Component::Node | Component::Cli => {
                log::info!("Successfully installed {component_str} v{version}");
                Self::check_installed_version(Component::Cli)?;
            }
            Component::Address | Component::Bech32 => {
                log::info!("Successfully installed {component_str} v{version}");
            }
            Component::Wallet => {
                log::info!("Successfully installed {component_str} {version}");
            }
        }
        Ok(())
    }

    pub fn uninstall_component(component: Component) -> Result<()> {
        let component = Self::component_to_string(component);
        log::info!("Uninstalling {component}");
        log::warn!("Not yet implemented");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_cardano_component() {
        let node = CardanoComponent::new(Component::Node);
        log::debug!("{node:#?}");
        let wallet = CardanoComponent::new(Component::Wallet);
        log::debug!("{wallet:#?}");
        let cli = CardanoComponent::new(Component::Cli);
        log::debug!("{cli:#?}");
        let address = CardanoComponent::new(Component::Address);
        log::debug!("{address:#?}");
        let bech32 = CardanoComponent::new(Component::Bech32);
        log::debug!("{bech32:#?}");
    }

    #[test]
    fn test_uninstall_component() {
        let result =
            CardanoComponent::uninstall_component(Component::Node).is_ok();
        assert!(result);
    }
}
