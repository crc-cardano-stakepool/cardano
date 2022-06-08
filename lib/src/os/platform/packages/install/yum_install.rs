use crate::{install_package, process_success};
use anyhow::Result;

pub async fn yum_install(package: &str) -> Result<()> {
    let cmd = format!("rpm -q {}", package);
    if !process_success(&cmd).await? {
        install_package("yum", package).await
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    // use crate::yum_install;
    #[tokio::test]
    #[ignore]
    async fn test_yum_install() {
        unimplemented!();
    }
}
