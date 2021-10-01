use crate::{
    check_distro, check_platform, install_distro_packages,
    install_mac_packages, print,
};
use anyhow::Result;

pub async fn setup_packages() -> Result<()> {
    let output = check_platform().await?;
    let platform = output.as_str().trim();
    match platform {
        "linux" | "Linux" => {
            print("green", "Detected linux")?;
            let output = check_distro().await?;
            let distro = output.as_str().trim();
            install_distro_packages(distro).await?;
        }
        "darwin" | "Darwin" => {
            print("green", "Detected macOS")?;
            install_mac_packages().await?
        }
        _ => panic!("Unsupported platform: {}", platform),
    }
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::setup_packages;
    #[tokio::test]
    #[ignore]
    async fn test_setup_packages() {
        unimplemented!();
    }
}
