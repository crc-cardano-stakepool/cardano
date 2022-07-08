use crate::{
    absolute_ref_path_to_string, async_command, async_command_pipe, check_env,
    VERSIONS_URL,
};
use anyhow::{anyhow, Result};
use std::path::Path;

pub fn check_cabal() -> Result<()> {
    log::debug!("Checking Cabal");
    let version = check_installed_cabal();
    match version {
        Ok(version) => {
            if compare_cabal(&version)? {
                log::info!("Installed Cabal v{version} is correct");
                return Ok(());
            }
            log::warn!("Cabal versions do not match");
            install_cabal()
        }
        Err(_) => install_cabal(),
    }
}

pub fn check_installed_cabal() -> Result<String> {
    log::debug!("Checking if Cabal is installed");
    let cabal = check_env("CABAL_BIN")?;
    let cabal_path = Path::new(&cabal);
    if cabal_path.is_file() {
        let cmd = format!("{cabal} -V | head -n1 | awk '{{print $3}}'");
        let installed_cabal = async_command_pipe(&cmd)?;
        let installed_cabal = installed_cabal.trim().to_string();
        log::debug!("Cabal v{installed_cabal} is installed");
        return Ok(installed_cabal);
    }
    Err(anyhow!("Cabal is not installed"))
}

pub fn compare_cabal(installed_cabal: &str) -> Result<bool> {
    log::debug!("Comparing installed Cabal v{installed_cabal} with required Cabal version to build a cardano node");
    let required = get_cabal_version()?;
    let installed = installed_cabal.trim().to_string();
    Ok(installed.eq(&required))
}

pub fn install_cabal() -> Result<()> {
    log::info!("Installing Cabal");
    let version = get_cabal_version()?;
    let ghcup = check_env("GHCUP_BIN")?;
    let cmd = format!("{ghcup} install cabal {version}");
    async_command(&cmd)?;
    let cmd = format!("{ghcup} set cabal {version}");
    async_command(&cmd)?;
    Ok(())
}

pub fn get_cabal_version() -> Result<String> {
    log::debug!("Getting required Cabal version to build a cardano node");
    let cmd = format!("curl -s {VERSIONS_URL} | tidy -i | grep '<code>cabal ' | awk '{{print $4}}' | awk -F '<' '{{print $1}}' | tail -n1");
    let cabal_version = async_command_pipe(&cmd)?;
    let cabal_version = cabal_version.trim();
    log::debug!("Required Cabal version: {cabal_version}");
    Ok(String::from(cabal_version))
}

pub fn update_cabal<P: AsRef<Path>>(path: P, cabal_path: P) -> Result<()> {
    log::info!("Updating Cabal");
    let path = absolute_ref_path_to_string(&path)?;
    let cabal_path = absolute_ref_path_to_string(&cabal_path)?;
    let cmd = format!("cd {path} && {cabal_path} update");
    async_command(&cmd)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::CABAL_VERSION;

    use super::*;

    #[test]
    #[ignore]
    fn test_check_cabal() -> Result<()> {
        check_cabal()?;
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_check_installed_cabal() -> Result<()> {
        check_installed_cabal()?;
        Ok(())
    }

    #[test]
    fn test_compare_cabal() -> Result<()> {
        assert_eq!(compare_cabal(CABAL_VERSION)?, true);
        assert_eq!(compare_cabal("3.6.0.0")?, false);
        Ok(())
    }

    #[test]
    fn test_get_cabal_version() -> Result<()> {
        let version = get_cabal_version()?;
        assert_eq!(version, CABAL_VERSION);
        Ok(())
    }
}
