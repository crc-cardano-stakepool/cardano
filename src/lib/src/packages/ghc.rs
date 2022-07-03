use crate::{async_command, async_command_pipe, check_env, file_exists, VERSIONS_URL};
use anyhow::{anyhow, Result};

pub async fn check_installed_ghc() -> Result<String> {
    log::info!("Checking GHC");
    let ghc = check_env("GHC_BIN")?;
    if file_exists(&ghc) {
        log::debug!("GHC is installed");
        let cmd = format!("{ghc} -V | awk {}", "'{print $8}'");
        let installed_ghc = async_command_pipe(&cmd).await?;
        let installed_ghc = installed_ghc.trim().to_string();
        log::debug!("GHC v{installed_ghc} is installed");
        Ok(installed_ghc)
    } else {
        Err(anyhow!("GHC is not installed"))
    }
}

pub async fn check_ghc() -> Result<()> {
    log::info!("Installing GHC if it is not installed");
    let ghc = check_installed_ghc().await;
    match ghc {
        Ok(ghc) => {
            if compare_ghc(&ghc).await? {
                log::info!("Installed GHC v{ghc} is correct");
                return Ok(());
            }
            log::warn!("GHC versions do not match");
            install_ghc().await
        }
        Err(_) => install_ghc().await,
    }
}

pub async fn compare_ghc(installed_ghc: &str) -> Result<bool> {
    log::info!("Comparing installed GHC v{installed_ghc} with the required GHC version to build a cardano node");
    let required = get_ghc_version().await?;
    let installed = installed_ghc.trim().to_string();
    Ok(installed.eq(&required))
}

pub async fn get_ghc_version() -> Result<String> {
    log::info!("Getting the correct GHC version to build a cardano node");
    let cmd = format!(
        "curl -s {VERSIONS_URL} | tidy -i | grep '<code>ghc ' | {} | {} | {}",
        "awk '{print $4}'", "awk -F '<' '{print $1}'", "tail -n1"
    );
    let ghc_version = async_command_pipe(&cmd).await?;
    let ghc_version = ghc_version.trim().to_string();
    Ok(ghc_version)
}

pub async fn install_ghc() -> Result<()> {
    log::info!("Installing GHC");
    let version = get_ghc_version().await?;
    let ghcup = check_env("GHCUP_BIN")?;
    let cmd = format!("{ghcup} install ghc {version}");
    async_command(&cmd).await?;
    let cmd = format!("{ghcup} set ghc {version}");
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set_env;

    #[tokio::test]
    #[ignore]
    async fn test_install_ghc() {
        unimplemented!();
    }

    #[tokio::test]
    async fn test_get_ghc_version() -> Result<()> {
        let version = get_ghc_version().await?;
        assert_eq!(version, "8.10.7");
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_get_ghcup_install_url() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_compare_ghc() -> Result<()> {
        compare_ghc("8.10.7").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_check_installed_ghc() -> Result<()> {
        let home_dir = dirs::home_dir().unwrap();
        let home_dir = home_dir.to_str().unwrap();
        let ghc_bin = format!("{home_dir}/.ghcup/bin/ghc");
        set_env("GHC_BIN", &ghc_bin);
        let version = check_installed_ghc().await;
        match version {
            Ok(version) => println!("{version}"),
            Err(err) => assert_eq!(err.to_string(), "GHC is not installed"),
        }
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_ghc() {
        unimplemented!();
    }
}
