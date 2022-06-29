use crate::{async_command_pipe, async_user_command, check_env, file_exists, VERSIONS_URL};
use anyhow::{anyhow, Result};

pub async fn check_installed_cabal() -> Result<String> {
    let cabal = check_env("CABAL_BIN")?;
    if file_exists(&cabal) {
        let cmd = format!("{cabal} -V | head -n1 | awk {}", "'{print $3}'");
        let installed_cabal = async_command_pipe(&cmd).await?;
        let installed_cabal = installed_cabal.trim().to_string();
        Ok(installed_cabal)
    } else {
        Err(anyhow!("Cabal is not installed"))
    }
}

pub async fn check_cabal() -> Result<()> {
    let cabal = check_installed_cabal().await;
    match cabal {
        Ok(cabal) => {
            if compare_cabal(&cabal).await? {
                Ok(())
            } else {
                install_cabal().await
            }
        }
        Err(_) => install_cabal().await,
    }
}

pub async fn compare_cabal(installed_cabal: &str) -> Result<bool> {
    let required = get_cabal_version().await?;
    let installed = installed_cabal.trim().to_string();
    Ok(installed.eq(&required))
}

pub async fn install_cabal() -> Result<()> {
    let version = get_cabal_version().await?;
    let ghcup = check_env("GHCUP_BIN")?;
    let cmd = format!("{ghcup} install cabal {version}");
    async_user_command(&cmd).await?;
    let cmd = format!("{ghcup} set cabal {version}");
    async_user_command(&cmd).await?;
    Ok(())
}

pub async fn get_cabal_version() -> Result<String> {
    let cmd = format!(
        "curl -s {VERSIONS_URL} | tidy -i | grep '<code>cabal ' | {} | {} | {}",
        "awk '{print $4}'", "awk -F '<' '{print $1}'", "tail -n1"
    );
    let cabal_version = async_command_pipe(&cmd).await?;
    let cabal_version = cabal_version.trim();
    Ok(String::from(cabal_version))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_check_cabal() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_installed_cabal() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_compare_cabal() {
        unimplemented!();
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
