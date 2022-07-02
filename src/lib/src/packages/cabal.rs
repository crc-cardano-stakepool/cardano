use crate::{async_command, async_command_pipe, check_env, file_exists, VERSIONS_URL};
use anyhow::{anyhow, Result};

pub async fn check_cabal() -> Result<()> {
    log::info!("Checking Cabal");
    let cabal = check_installed_cabal().await;
    match cabal {
        Ok(cabal) => {
            if compare_cabal(&cabal).await? {
                log::info!("Correct Cabal version is installed");
                return Ok(());
            }
            log::warn!("Cabal versions do not match");
            install_cabal().await
        }
        Err(_) => install_cabal().await,
    }
}

pub async fn check_installed_cabal() -> Result<String> {
    log::info!("Checking if Cabal is installed");
    let cabal = check_env("CABAL_BIN")?;
    if file_exists(&cabal) {
        let cmd = format!("{cabal} -V | head -n1 | awk '{{print $3}}'");
        let installed_cabal = async_command_pipe(&cmd).await?;
        let installed_cabal = installed_cabal.trim().to_string();
        log::debug!("Installed Cabal v{installed_cabal}");
        return Ok(installed_cabal);
    }
    Err(anyhow!("Cabal is not installed"))
}

pub async fn compare_cabal(installed_cabal: &str) -> Result<bool> {
    log::info!("Comparing installed Cabal v{installed_cabal} with required Cabal version to build a cardano node");
    let required = get_cabal_version().await?;
    let installed = installed_cabal.trim().to_string();
    Ok(installed.eq(&required))
}

pub async fn install_cabal() -> Result<()> {
    log::info!("Installing Cabal");
    let version = get_cabal_version().await?;
    let ghcup = check_env("GHCUP_BIN")?;
    let cmd = format!("{ghcup} install cabal {version}");
    async_command(&cmd).await?;
    let cmd = format!("{ghcup} set cabal {version}");
    async_command(&cmd).await?;
    Ok(())
}

pub async fn get_cabal_version() -> Result<String> {
    log::info!("Getting required Cabal version to build a cardano node");
    let cmd = format!(
        "curl -s {VERSIONS_URL} | tidy -i | grep '<code>cabal ' | {} | {} | {}",
        "awk '{print $4}'", "awk -F '<' '{print $1}'", "tail -n1"
    );
    let cabal_version = async_command_pipe(&cmd).await?;
    let cabal_version = cabal_version.trim();
    log::debug!("Required Cabal version: {cabal_version}");
    Ok(String::from(cabal_version))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_check_cabal() -> Result<()> {
        check_cabal().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_installed_cabal() -> Result<()> {
        check_installed_cabal().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_compare_cabal() -> Result<()> {
        let version = "3.6.2.0";
        assert_eq!(compare_cabal(version).await?, true);
        let version = "3.6.0.0";
        assert_eq!(compare_cabal(version).await?, false);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_cabal_version() -> Result<()> {
        let version = get_cabal_version().await?;
        assert_eq!(version, "3.6.2.0");
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_install_cabal() {
        unimplemented!();
    }
}
