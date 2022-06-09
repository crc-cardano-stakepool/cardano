use crate::{async_command_pipe, async_user_command, check_env, file_exists, print, VERSIONS_URL};
use anyhow::Result;
use async_recursion::async_recursion;

#[async_recursion]
pub async fn check_installed_cabal() -> Result<String> {
    let cabal = check_env("CABAL_BIN")?;
    if file_exists(&cabal) {
        let cmd = format!("{cabal} -V | head -n1 | awk {}", "'{print $3}'");
        let installed_cabal = async_command_pipe(&cmd).await?;
        let installed_cabal = installed_cabal.trim().to_string();
        Ok(installed_cabal)
    } else {
        install_cabal().await?;
        check_installed_cabal().await
    }
}

pub async fn check_cabal() -> Result<()> {
    print("", "Checking Cabal")?;
    let cabal = check_installed_cabal().await?;
    if compare_cabal(&cabal).await? {
        print("green", "Cabal is installed")
    } else {
        let msg = format!("Currently Cabal v{cabal} is installed, installing correct version of Cabal");
        print("yellow", &msg)?;
        install_cabal().await
    }
}

pub async fn compare_cabal(installed_cabal: &str) -> Result<bool> {
    let version = get_cabal_version().await?;
    Ok(installed_cabal.eq(&version))
}

pub async fn install_cabal() -> Result<()> {
    print("", "Installing Cabal")?;
    let version = get_cabal_version().await?;
    let msg = format!("Installing Cabal v{version}");
    print("", &msg)?;
    let ghcup = check_env("GHCUP_BIN")?;
    let cmd = format!("{ghcup} install cabal {version}");
    async_user_command(&cmd).await?;
    let cmd = format!("{ghcup} set cabal {version}");
    async_user_command(&cmd).await?;
    print("green", "Successfully installed Cabal")
}

pub async fn get_cabal_version() -> Result<String> {
    let cmd = format!(
        "curl -s {VERSIONS_URL} | fold -w100 | grep '<code>cabal ' | sed 's/^.*cabal //' | {} | {}",
        "awk -F '<' '{print $1}'", "tail -n1"
    );
    let cabal_version = async_command_pipe(&cmd).await?;
    let cabal_version = cabal_version.trim();
    println!("{cabal_version}");
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
