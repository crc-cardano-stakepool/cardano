use crate::{install_package, pipe};
use anyhow::{anyhow, Result};

pub async fn apt_install(package: &str) -> Result<()> {
    let cmd = format!("dpkg -s {}", package.trim());
    let piped_cmd = "grep installed";
    if let Ok(result) = pipe(&cmd, piped_cmd).await {
        if result.trim().is_empty() {
            install_package("apt", package).await
        } else {
            Ok(())
        }
    } else {
        Err(anyhow!("Failed installing {}", package))
    }
}

#[cfg(test)]
mod test {
    // use crate::apt_install;
    #[tokio::test]
    #[ignore]
    async fn test_apt_install() {
        unimplemented!();
    }
}
