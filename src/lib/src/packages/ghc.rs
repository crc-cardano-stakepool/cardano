use crate::{Environment, Executer, VERSIONS_URL};
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Ghc {
    pub binary_name: String,
    pub latest_version: String,
    pub installed_version: Option<String>,
    pub bin_path: Option<PathBuf>,
}

impl Default for Ghc {
    fn default() -> Self {
        let binary_name = "ghc".to_string();
        let installed_version = Self::check_installed_version().ok();
        let latest_version = Self::get_version().unwrap();
        let bin_path =
            Environment::check_env("GHC_BIN").map(PathBuf::from).ok();
        Self {
            binary_name,
            latest_version,
            installed_version,
            bin_path,
        }
    }
}

impl Ghc {
    pub fn get_name(&self) -> String {
        self.binary_name.clone()
    }
    pub fn get_latest_version(&self) -> String {
        self.latest_version.clone()
    }
    pub fn get_installed_version(&self) -> Option<String> {
        self.installed_version.clone()
    }
    pub fn get_bin_path(&self) -> Option<PathBuf> {
        self.bin_path.clone()
    }
}

impl Ghc {
    pub fn check_installed_version() -> Result<String> {
        log::debug!("Checking GHC");
        let ghc = Environment::check_env("GHC_BIN")?;
        let ghc_path = Path::new(&ghc);
        if ghc_path.is_file() {
            log::debug!("GHC is installed");
            let cmd = format!("{ghc} -V | awk {}", "'{print $8}'");
            let installed_ghc = Executer::capture(&cmd)?;
            let installed_ghc = installed_ghc.trim().to_string();
            log::debug!("GHC v{installed_ghc} is installed");
            Ok(installed_ghc)
        } else {
            Err(anyhow!("GHC is not installed"))
        }
    }

    pub fn check() -> Result<()> {
        log::debug!("Installing GHC if it is not installed");
        Self::check_installed_version()
            .map(|ghc| {
                if Self::compare(&ghc)? {
                    log::info!("Installed GHC v{ghc} is correct");
                    return Ok(());
                }
                log::warn!("GHC versions do not match");
                Self::install()
            })
            .map_err(|_| Self::install())
            .unwrap()
    }

    pub fn compare(installed_ghc: &str) -> Result<bool> {
        log::debug!("Comparing installed GHC v{installed_ghc} with the required GHC version to build a cardano node");
        let required = Self::get_version()?;
        let installed = installed_ghc.trim().to_string();
        Ok(installed.eq(&required))
    }

    pub fn get_version() -> Result<String> {
        log::debug!("Getting the correct GHC version to build a cardano node");
        let cmd = format!(
            "
            curl -s {VERSIONS_URL} | \
            tidy -i | \
            grep '<code>ghc ' | \
            awk '{{print $4}}' | \
            awk -F '<' '{{print $1}}' | \
            tail -n1"
        );
        let ghc_version = Executer::capture(&cmd)?;
        let ghc_version = ghc_version.trim().to_string();
        Ok(ghc_version)
    }

    pub fn install() -> Result<()> {
        log::info!("Installing GHC");
        let version = Self::get_version()?;
        let ghcup = Environment::check_env("GHCUP_BIN")?;
        let cmd = format!("{ghcup} install ghc {version}");
        Executer::exec(&cmd)?;
        let cmd = format!("{ghcup} set ghc {version}");
        Executer::exec(&cmd)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::GHC_VERSION;

    #[test]
    fn test_ghc() {
        let ghc = Ghc::default();
        log::debug!("{ghc:#?}");
    }

    #[test]
    fn test_get_ghc_version() -> Result<()> {
        let version = Ghc::get_version()?;
        assert_eq!(version, GHC_VERSION);
        Ok(())
    }
    #[test]
    #[ignore]
    fn test_compare_ghc() -> Result<()> {
        assert_eq!(Ghc::compare(GHC_VERSION)?, true);
        assert_eq!(Ghc::compare("8.10.2")?, false);
        assert_eq!(Ghc::compare("8.10.4")?, false);
        Ok(())
    }

    #[test]
    fn test_check_installed_ghc() -> Result<()> {
        let home_dir = dirs::home_dir().unwrap();
        let home_dir = home_dir.to_str().unwrap();
        let ghc_bin = format!("{home_dir}/.ghcup/bin/ghc");
        Environment::set_env("GHC_BIN", &ghc_bin);
        let version = Ghc::check_installed_version();
        match version {
            Ok(version) => println!("{version}"),
            Err(err) => assert_eq!(err.to_string(), "GHC is not installed"),
        }
        Ok(())
    }
}
