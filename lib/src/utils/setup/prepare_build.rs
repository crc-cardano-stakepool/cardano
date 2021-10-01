use crate::{
    check_dir, check_work_dir, install_build_dependencies, print,
    setup_packages, setup_shell,
};
use anyhow::Result;

pub async fn prepare_build() -> Result<()> {
    print("", "Preparing build")?;
    check_dir(&check_work_dir().await?).await?;
    setup_packages().await?;
    setup_shell().await?;
    install_build_dependencies().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::prepare_build;
    #[tokio::test]
    #[ignore]
    async fn test_prepare_build() {
        unimplemented!();
    }
}
