use crate::{async_command_pipe, async_user_command, check_env, file_exists, print, VERSIONS_URL};
use anyhow::{anyhow, Result};

pub async fn check_installed_ghc() -> Result<String> {
    let ghc = check_env("GHC_BIN")?;
    if file_exists(&ghc) {
        let cmd = format!("{ghc} -V | awk {}", "'{print $8}'");
        let installed_ghc = async_command_pipe(&cmd).await?;
        let installed_ghc = installed_ghc.trim().to_string();
        Ok(installed_ghc)
    } else {
        Err(anyhow!("GHC is not installed"))
    }
}

pub async fn check_ghc() -> Result<()> {
    print("", "Checking GHC")?;
    let ghc = check_installed_ghc().await;
    match ghc {
        Ok(ghc) => {
            if compare_ghc(&ghc).await? {
                print("green", "GHC is installed")
            } else {
                let msg = format!("Currently GHC v{ghc} is installed, installing correct version of GHC");
                print("yellow", &msg)?;
                install_ghc().await
            }
        }
        Err(_) => install_ghc().await,
    }
}

pub async fn compare_ghc(installed_ghc: &str) -> Result<bool> {
    let version = get_ghc_version().await?;
    Ok(installed_ghc.eq(&version))
}

pub async fn get_ghc_version() -> Result<String> {
    let cmd = format!(
        "curl -s {VERSIONS_URL} | tidy -i | grep '<code>ghc ' | {} | {} | {}",
        "awk '{print $4}'", "awk -F '<' '{print $1}'", "tail -n1"
    );
    let ghc_version = async_command_pipe(&cmd).await?;
    let ghc_version = ghc_version.trim();
    Ok(String::from(ghc_version))
}

pub async fn install_ghc() -> Result<()> {
    let version = get_ghc_version().await?;
    let msg = format!("Installing GHC v{version}",);
    print("", &msg)?;
    let ghcup = check_env("GHCUP_BIN")?;
    let cmd = format!("{ghcup} install ghc {version}");
    async_user_command(&cmd).await?;
    let cmd = format!("{ghcup} set ghc {version}");
    async_user_command(&cmd).await?;
    print("green", "Successfully installed GHC")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{set_env, check_home_dir};

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
    async fn test_compare_ghc() {
        unimplemented!();
    }

    #[tokio::test]
    async fn test_check_installed_ghc() -> Result<()> {
        let home_dir = check_home_dir().await?;
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
