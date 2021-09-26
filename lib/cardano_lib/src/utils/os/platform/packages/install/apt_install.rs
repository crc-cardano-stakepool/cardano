use crate::{install_package, pipe, print};
use anyhow::Result;

pub async fn apt_install(package: &str) -> Result<()> {
    let cmd = format!("dpkg -s {}", package.trim());
    let piped_cmd = "grep installed";
    let output = pipe(&cmd, piped_cmd).await;
    match output {
        Ok(result) => {
            if result.trim().is_empty() {
                install_package("apt", package).await?;
            } else {
                let msg = format!("{} is installed", package);
                print("green", &msg)?;
            }
        }
        Err(_) => {
            let msg = format!("Failed checking {}", package);
            print("red", &msg)?;
        }
    }
    Ok(())
}
