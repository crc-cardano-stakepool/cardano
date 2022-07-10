use crate::{Environment, Executer, FileSystem, VERSIONS_URL};
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Cabal {
    pub binary_name: String,
    pub latest_version: String,
    pub installed_version: Option<String>,
    pub bin_path: Option<PathBuf>,
}

impl Default for Cabal {
    fn default() -> Self {
        let binary_name = "cabal".to_string();
        let installed_version = Self::check_installed_version().ok();
        let latest_version = Self::get_version().unwrap();
        let bin_path =
            Environment::check_env("CABAL_BIN").map(PathBuf::from).ok();
        Self {
            binary_name,
            latest_version,
            installed_version,
            bin_path,
        }
    }
}

impl Cabal {
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

impl Cabal {
    pub fn check() -> Result<()> {
        log::debug!("Checking Cabal");
        Self::check_installed_version()
            .map(|version| {
                if Self::compare(&version)? {
                    log::info!("Installed Cabal v{version} is correct");
                    return Ok(());
                }
                log::warn!("Cabal versions do not match");
                Self::install()
            })
            .map_err(|_| Self::install())
            .unwrap()
    }

    pub fn check_installed_version() -> Result<String> {
        log::debug!("Checking if Cabal is installed");
        let cabal = Environment::check_env("CABAL_BIN")?;
        let cabal_path = Path::new(&cabal);
        if !cabal_path.is_file() {
            return Err(anyhow!("Cabal is not installed"));
        }
        let cmd = format!("{cabal} -V | head -n1 | awk '{{print $3}}'");
        Executer::capture(&cmd).map(|version| {
            log::debug!("Cabal v{version} is installed");
            version
        })
    }

    pub fn compare(installed_cabal: &str) -> Result<bool> {
        let msg = format!(
            "Comparing installed Cabal v{installed_cabal} \
            with required Cabal version to build a cardano node"
        );
        log::debug!("{msg}");
        let installed = installed_cabal.trim().to_string();
        Self::get_version().map(|required| installed.eq(&required))
    }

    pub fn install() -> Result<()> {
        log::info!("Installing Cabal");
        let version = Self::get_version()?;
        let ghcup = Environment::check_env("GHCUP_BIN")?;
        let cmd = format!("{ghcup} install cabal {version}");
        Executer::exec(&cmd)?;
        let cmd = format!("{ghcup} set cabal {version}");
        Executer::exec(&cmd)
    }

    pub fn get_version() -> Result<String> {
        log::debug!("Getting required Cabal version to build a cardano node");
        let cmd = format!(
            "
            curl -s {VERSIONS_URL} | \
            tidy -i | \
            grep '<code>cabal ' | \
            awk '{{print $4}}' | \
            awk -F '<' '{{print $1}}' | \
            tail -n1"
        );
        Executer::capture(&cmd).map(|version| {
            log::debug!("Required Cabal version: {version}");
            version
        })
    }

    pub fn update<P: AsRef<Path>>(path: P, cabal_path: P) -> Result<()> {
        log::info!("Updating Cabal");
        let path = FileSystem::absolute_ref_path_to_string(&path)?;
        let cabal_path = FileSystem::absolute_ref_path_to_string(&cabal_path)?;
        let cmd = format!("cd {path} && {cabal_path} update");
        Executer::exec(&cmd)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CABAL_VERSION;

    #[test]
    #[ignore]
    fn test_cabal() {
        let cabal = Cabal::default();
        log::debug!("{cabal:#?}");
    }

    #[test]
    #[ignore]
    fn test_check_cabal() -> Result<()> {
        Cabal::check()?;
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_check_installed_cabal() -> Result<()> {
        Cabal::check_installed_version()?;
        Ok(())
    }

    #[test]
    fn test_compare_cabal() -> Result<()> {
        assert_eq!(Cabal::compare(CABAL_VERSION)?, true);
        assert_eq!(Cabal::compare("3.6.0.0")?, false);
        Ok(())
    }

    #[test]
    fn test_get_cabal_version() -> Result<()> {
        let version = Cabal::get_version()?;
        assert_eq!(version, CABAL_VERSION);
        Ok(())
    }
}
