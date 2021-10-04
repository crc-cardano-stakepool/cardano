use crate::{print, spinner_cmd};
use anyhow::{anyhow, Result};

pub async fn install_package(
    package_manager: &str,
    package: &str,
) -> Result<()> {
    let msg = format!("{} is not installed", package);
    print("red", &msg)?;
    let cmd = format!("sudo {} install {} -y", package_manager, package);
    let exec_msg: &'static str = format!("Installing {}", package);
    let finish_msg = format!("Finished installing {}", package);
    let process = spinner_cmd(&cmd, &exec_msg, &finish_msg).await;
    match process {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(anyhow!("Failed installing {} with error: {}", package, e))
        }
    }
}

#[cfg(test)]
mod test {
    // use crate::install_package;
    #[tokio::test]
    #[ignore]
    async fn test_install_package() {
        unimplemented!();
    }
}
