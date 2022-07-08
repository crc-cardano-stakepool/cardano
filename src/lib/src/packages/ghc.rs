use crate::{async_command, async_command_pipe, check_env, VERSIONS_URL};
use anyhow::{anyhow, Result};
use std::path::Path;

pub fn check_installed_ghc() -> Result<String> {
    log::debug!("Checking GHC");
    let ghc = check_env("GHC_BIN")?;
    let ghc_path = Path::new(&ghc);
    if ghc_path.is_file() {
        log::debug!("GHC is installed");
        let cmd = format!("{ghc} -V | awk {}", "'{print $8}'");
        let installed_ghc = async_command_pipe(&cmd)?;
        let installed_ghc = installed_ghc.trim().to_string();
        log::debug!("GHC v{installed_ghc} is installed");
        Ok(installed_ghc)
    } else {
        Err(anyhow!("GHC is not installed"))
    }
}

pub fn check_ghc() -> Result<()> {
    log::debug!("Installing GHC if it is not installed");
    let ghc = check_installed_ghc();
    match ghc {
        Ok(ghc) => {
            if compare_ghc(&ghc)? {
                log::info!("Installed GHC v{ghc} is correct");
                return Ok(());
            }
            log::warn!("GHC versions do not match");
            install_ghc()
        }
        Err(_) => install_ghc(),
    }
}

pub fn compare_ghc(installed_ghc: &str) -> Result<bool> {
    log::debug!("Comparing installed GHC v{installed_ghc} with the required GHC version to build a cardano node");
    let required = get_ghc_version()?;
    let installed = installed_ghc.trim().to_string();
    Ok(installed.eq(&required))
}

pub fn get_ghc_version() -> Result<String> {
    log::debug!("Getting the correct GHC version to build a cardano node");
    let cmd = format!("curl -s {VERSIONS_URL} | tidy -i | grep '<code>ghc ' | awk '{{print $4}}' | awk -F '<' '{{print $1}}' | tail -n1");
    let ghc_version = async_command_pipe(&cmd)?;
    let ghc_version = ghc_version.trim().to_string();
    Ok(ghc_version)
}

pub fn install_ghc() -> Result<()> {
    log::info!("Installing GHC");
    let version = get_ghc_version()?;
    let ghcup = check_env("GHCUP_BIN")?;
    let cmd = format!("{ghcup} install ghc {version}");
    async_command(&cmd)?;
    let cmd = format!("{ghcup} set ghc {version}");
    async_command(&cmd)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{set_env, GHC_VERSION};

    #[test]
    fn test_get_ghc_version() -> Result<()> {
        let version = get_ghc_version()?;
        assert_eq!(version, GHC_VERSION);
        Ok(())
    }
    #[test]
    #[ignore]
    fn test_compare_ghc() -> Result<()> {
        assert_eq!(compare_ghc(GHC_VERSION)?, true);
        assert_eq!(compare_ghc("8.10.2")?, false);
        assert_eq!(compare_ghc("8.10.4")?, false);
        Ok(())
    }

    #[test]
    fn test_check_installed_ghc() -> Result<()> {
        let home_dir = dirs::home_dir().unwrap();
        let home_dir = home_dir.to_str().unwrap();
        let ghc_bin = format!("{home_dir}/.ghcup/bin/ghc");
        set_env("GHC_BIN", &ghc_bin);
        let version = check_installed_ghc();
        match version {
            Ok(version) => println!("{version}"),
            Err(err) => assert_eq!(err.to_string(), "GHC is not installed"),
        }
        Ok(())
    }
}
